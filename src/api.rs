use warp::{Filter, Reply, Rejection};
use std::fs::{self, create_dir_all};
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use chrono::prelude::*;
use serde::Serialize;
use crate::crypto::{encrypt_data, decrypt_data};
use crate::model::{TranslateRequest, ApiKeys, TableSchema, ColumnSchema, TableMeta, TableIndex, IndexEntry};
use crate::utils::{with_keys, load_lang_map, encode_cell, decode_cell};

// نوع خطای سفارشی
#[derive(Debug, Serialize)]
struct CustomError {
    status: String,
    message: String,
}

impl warp::reject::Reject for CustomError {}


pub async fn start_server() {
    let keys_data = fs::read_to_string("api_keys.json").map_err(|e| {
        eprintln!("Failed to read api_keys.json: {}", e);
        std::process::exit(1);
    }).unwrap();
    let parsed: ApiKeys = serde_json::from_str(&keys_data).map_err(|e| {
        eprintln!("Failed to parse api_keys.json: {}", e);
        std::process::exit(1);
    }).unwrap();
    let key_set: HashSet<String> = parsed.keys.into_iter().collect();
    let keys = Arc::new(key_set);

    let post_route = warp::post()
        .and(warp::path!("api" / "store"))
        .and(warp::header::optional::<String>("authorization"))
        .and(warp::body::json())
        .and(with_keys(keys.clone()))
        .and_then(handle_translate);

    let get_route = warp::get()
        .and(warp::path!("api" / "store" / String / String))
        .and(with_keys(keys.clone()))
        .and_then(handle_get_file);

    let routes = post_route.or(get_route);
    println!("🚀 DonutDB API running on http://localhost:4040");
    warp::serve(routes).run(([0, 0, 0, 0], 4040)).await;
}

async fn handle_translate(
    auth_header: Option<String>,
    req: TranslateRequest,
    keys: Arc<HashSet<String>>,
) -> Result<impl Reply, Rejection> {
    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => h.trim_start_matches("Bearer ").trim().to_string(),
        _ => return Err(warp::reject::custom(CustomError {
            status: "\u{274C} Unauthorized".to_string(),
            message: "Missing or invalid Authorization header".to_string(),
        })),
    };

    if !keys.contains(&token) {
        return Err(warp::reject::custom(CustomError {
            status: "\u{274C} Invalid API Key".to_string(),
            message: "Provided API key is not valid".to_string(),
        }));
    }

    let lang_map = load_lang_map();

    // Encode داده‌ها
    let translated_data: Vec<Vec<String>> = req.data.iter()
        .map(|row| row.iter().map(|cell| encode_cell(cell, &lang_map)).collect())
        .collect();

    // مسیر جدول
    let table_dir = PathBuf::from(format!("donutdb/{}/data", token));
    create_dir_all(&table_dir).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Failed to create table directory".to_string(),
        message: format!("Directory creation failed: {}", e),
    }))?;

    // ذخیره داده‌های ستونی
    for (col_idx, col_name) in req.slot.iter().enumerate() {
        let mut col_data = String::new();
        for row in &translated_data {
            col_data.push_str(&row[col_idx]);
            col_data.push('\n');
        }
        let encrypted_col = encrypt_data(&col_data, &token).map_err(|e| warp::reject::custom(CustomError {
            status: "\u{274C} Encryption Failed".to_string(),
            message: format!("Encryption failed: {}", e),
        }))?;
        let col_file = table_dir.join(format!("{}.odb.part1", col_name));
        fs::write(&col_file, encrypted_col).map_err(|e| warp::reject::custom(CustomError {
            status: "\u{274C} Write Failed".to_string(),
            message: format!("Write failed: {}", e),
        }))?;
    }

    // ذخیره اسکیما
    let schema = TableSchema {
        columns: req.slot.iter().map(|name| ColumnSchema {
            name: name.clone(),
            r#type: "string".to_string(),
        }).collect(),
    };
    let schema_data = serde_json::to_string(&schema).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Serialization Failed".to_string(),
        message: format!("Schema serialization failed: {}", e),
    }))?;
    let encrypted_schema = encrypt_data(&schema_data, &token).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Encryption Failed".to_string(),
        message: format!("Encryption failed: {}", e),
    }))?;
    let schema_file = PathBuf::from(format!("donutdb/{}/schema.oschema", token));
    fs::write(&schema_file, encrypted_schema).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Write Failed".to_string(),
        message: format!("Write failed: {}", e),
    }))?;

    // ذخیره متادیتا
    let now = Utc::now().to_rfc3339();
    let meta = TableMeta {
        table_name: req.dataset.clone(),
        record_count: req.data.len(),
        partition_count: 1,
        created_at: now.clone(),
        updated_at: now,
    };
    let meta_data = serde_json::to_string(&meta).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Serialization Failed".to_string(),
        message: format!("Meta serialization failed: {}", e),
    }))?;
    let encrypted_meta = encrypt_data(&meta_data, &token).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Encryption Failed".to_string(),
        message: format!("Encryption failed: {}", e),
    }))?;
    let meta_file = PathBuf::from(format!("donutdb/{}/meta.ometa", token));
    fs::write(&meta_file, encrypted_meta).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Write Failed".to_string(),
        message: format!("Write failed: {}", e),
    }))?;

    // ذخیره ایندکس
    let index = TableIndex {
        primary_key: req.slot[0].clone(),
        indexes: req.data.iter().enumerate().map(|(i, _)| IndexEntry {
            id: i as u64 + 1,
            offset: (i * 10) as u64,
            partition: format!("{}.odb.part1", req.slot[0]),
        }).collect(),
    };
    let index_data = serde_json::to_string(&index).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Serialization Failed".to_string(),
        message: format!("Index serialization failed: {}", e),
    }))?;
    let encrypted_index = encrypt_data(&index_data, &token).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Encryption Failed".to_string(),
        message: format!("Encryption failed: {}", e),
    }))?;
    let index_file = PathBuf::from(format!("donutdb/{}/index.oidx", token));
    fs::write(&index_file, encrypted_index).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Write Failed".to_string(),
        message: format!("Write failed: {}", e),
    }))?;

    Ok(warp::reply::json(&serde_json::json!({
        "status": "\u{2705} Stored",
        "dataset": req.dataset
    })))
}

async fn handle_get_file(
    file: String,
    key: String,
    keys: Arc<HashSet<String>>,
) -> Result<impl Reply, Rejection> {
    if !keys.contains(&key) {
        return Err(warp::reject::custom(CustomError {
            status: "\u{274C} Invalid API Key".to_string(),
            message: "Provided API key is not valid".to_string(),
        }));
    }

    // خواندن متادیتا
    let meta_file = PathBuf::from(format!("donutdb/{}/meta.ometa", key));
    let meta_data = fs::read(&meta_file).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} File Not Found".to_string(),
        message: format!("Meta file not found: {}", e),
    }))?;
    let meta_content = decrypt_data(&meta_data, &key).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Decryption Failed".to_string(),
        message: format!("Decryption failed for meta: {}", e),
    }))?;
    let meta: TableMeta = serde_json::from_str(&meta_content).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Deserialization Failed".to_string(),
        message: format!("Meta deserialization failed: {}", e),
    }))?;

    // خواندن اسکیما
    let schema_file = PathBuf::from(format!("donutdb/{}/schema.oschema", key));
    let schema_data = fs::read(&schema_file).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} File Not Found".to_string(),
        message: format!("Schema file not found: {}", e),
    }))?;
    let schema_content = decrypt_data(&schema_data, &key).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Decryption Failed".to_string(),
        message: format!("Decryption failed for schema: {}", e),
    }))?;
    let schema: TableSchema = serde_json::from_str(&schema_content).map_err(|e| warp::reject::custom(CustomError {
        status: "\u{274C} Deserialization Failed".to_string(),
        message: format!("Schema deserialization failed: {}", e),
    }))?;

    // خواندن داده‌ها
    let mut columns_data = Vec::new();
    for col in &schema.columns {
        let col_file = PathBuf::from(format!("donutdb/{}/data/{}.odb.part1", key, col.name));
        let col_data = fs::read(&col_file).map_err(|e| warp::reject::custom(CustomError {
            status: "\u{274C} File Not Found".to_string(),
            message: format!("Column file not found: {} ({})", col.name, e),
        }))?;
        let col_content = decrypt_data(&col_data, &key).map_err(|e| warp::reject::custom(CustomError {
            status: "\u{274C} Decryption Failed".to_string(),
            message: format!("Decryption failed for column {}: {}", col.name, e),
        }))?;
        columns_data.push(col_content.split('\n').map(String::from).collect::<Vec<String>>());
    }

    // بازسازی داده‌ها
    let lang_map = load_lang_map();
    let reverse_map: HashMap<String, char> = lang_map.iter().map(|(k, v)| (v.clone(), *k)).collect();
    let mut data = Vec::new();
    for row_idx in 0..meta.record_count {
        let mut row = Vec::new();
        for col_data in &columns_data {
            if let Some(cell) = col_data.get(row_idx) {
                row.push(decode_cell(cell, &reverse_map));
            }
        }
        data.push(row);
    }

    let restored = format!(
        "dataset {}({}) {{\n    {}\n{}\n}}",
        meta.table_name,
        schema.columns.iter().map(|c| format!("SLOT:{}", c.name)).collect::<Vec<_>>().join(", "),
        schema.columns.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(","),
        data.iter().map(|row| format!("    {}", row.join(","))).collect::<Vec<_>>().join("\n")
    );

    Ok(warp::reply::json(&serde_json::json!({
        "status": "\u{2705} Success",
        "filename": file,
        "data": restored
    })))
}


// Include the handle_translate and handle_get_file functions here (truncated for space)