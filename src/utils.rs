use warp::Filter;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::fs;

pub fn with_keys(keys: Arc<HashSet<String>>) -> impl Filter<Extract = (Arc<HashSet<String>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || keys.clone())
}

pub fn load_lang_map() -> HashMap<char, String> {
    let json_str = fs::read_to_string("lang.json").expect("Cannot read lang.json");
    let raw_map: HashMap<String, String> = serde_json::from_str(&json_str).unwrap();
    raw_map.into_iter().map(|(k, v)| (k.chars().next().unwrap(), v)).collect()
}

pub fn encode_cell(cell: &str, map: &HashMap<char, String>) -> String {
    cell.chars().map(|c| map.get(&c).cloned().unwrap_or(c.to_string())).collect::<Vec<_>>().join("")
}

pub fn decode_cell(cell: &str, reverse_map: &HashMap<String, char>) -> String {
    let mut buffer = String::new();
    let mut result = String::new();

    for ch in cell.chars() {
        buffer.push(ch);
        if let Some(c) = reverse_map.get(&buffer) {
            result.push(*c);
            buffer.clear();
        }
    }
    result
}