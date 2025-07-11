# DonutDB 🥯

**DonutDB** is a lightweight, encrypted, column-oriented database engine written in **Rust**. It focuses on high performance, privacy, and simplicity. DonutDB is perfect for secure applications that demand structured tabular storage with minimal overhead.

> Designed and developed by [@unknownmsv](https://github.com/unknownmsv)

---

## ✨ Features

* **Encrypted Columnar Storage** with AES-256 encryption
* **50MB Partitioning** for performance and scalability
* **REST API** powered by Warp for easy access
* **Schema & Index Management** with encrypted files
* **Custom Language Mapping** for efficient encoding
* **Open-Source with Ethical Licensing**

---

## 🧰 Architecture

```
donutdb/
 └── <api_key>/
     ├── meta.ometa        # Metadata (Encrypted)
     ├── schema.oschema    # Schema definitions (Encrypted)
     ├── index.oidx        # Indexes for fast lookups (Encrypted)
     └── data/
         ├── id.odb.part1   # Partitioned columnar data (Encrypted)
         └── ...
```

All files are encrypted using AES-256. API keys are used to derive cryptographic keys.

---

## ⚙️ Installation

### Prerequisites

* Rust (via [rustup.rs](https://rustup.rs/))
* Git

### Steps

```bash
git clone https://github.com/unknownmsv/DonutDB.git
cd DonutDB
```

Create the config files:

**api\_keys.json**

```json
{"keys": ["test-key"]}
```

**lang.json**

```json
{"1": "01", "2": "02", "a": "alice", "b": "bob"}
```

Then build:

```bash
cargo build --release
```

---

## 🚩 Usage

### Start the server

```bash
target/release/donutdb
```

### POST /api/store

Store tabular data.

**Headers:**

* Authorization: Bearer test-key
* Content-Type: application/json

**Body:**

```json
{
  "dataset": "users",
  "slot": ["id", "username"],
  "data": [["1", "alice"], ["2", "bob"]]
}
```

### Example:

```bash
curl -X POST http://localhost:4040/api/store \
  -H "Authorization: Bearer test-key" \
  -H "Content-Type: application/json" \
  -d '{
    "dataset": "users",
    "slot": ["id", "username"],
    "data": [["1", "alice"], ["2", "bob"]]
  }'
```

### GET /api/store/<dataset>/\<api\_key>

Retrieve data by dataset name and API key.

```bash
curl http://localhost:4040/api/store/users/test-key
```

---

DonutDB is licensed under the AGPL-3.0 License with extra attribution requirements.  
See [LICENSE](./LICENSE) for full terms.

---

## 🧑‍💻 Contact

* GitHub: [@unknownmsv](https://github.com/unknownmsv)
* Discord: `@unknownmsv`

---

## 📊 Roadmap

* [x] AES-256 full encryption
* [x] Indexing system
* [ ] Compression layer
* [ ] SQL-like query interface
* [ ] Cross-platform binary packaging

---

## 👍 Acknowledgments

Built with ❤️ and Rust.

Thanks to the open-source community for support and inspiration.
