# DonutDB 🥯

**DonutDB** is a lightweight, encrypted, columnar database written in Rust, designed for efficient and secure storage of tabular data in a columnar format. With support for 50MB partitioning, AES-256 encryption, and a robust schema management system, DonutDB is ideal for personal projects, research, and commercial applications requiring high-performance data storage with a focus on security and scalability.

Built by **[unknownmsv](https://github.com/unknownmsv)**, DonutDB aims to provide a simple yet powerful database solution that combines modern cryptographic techniques with Rust's safety guarantees.

---

## Table of Contents
- [Features](#)
- [Architecture](#)
- [File Structure](#)
- [Installation](#)
- [Usage](#)
- [API Documentation](#)
- [License](#)
- [Contributing](#)
- [Contact](#)
- [Roadmap](#)
- [Acknowledgments](#)

---

## Features

- **Columnar Storage**: Stores data in columns for efficient compression and retrieval, optimized for analytical queries.
- **50MB Partitioning**: Automatically partitions large datasets into 50MB chunks for scalability and performance.
- **AES-256 Encryption**: Secures all data, schemas, schemas, and metadata, and metadata with AES-256 encryption using API key-derived keys.
- **Schema and Metadata Management**: Supports flexible schema definitions stored in encrypted `schema.oschema` files.
- **Indexing**: Provides fast data lookup with encrypted index files (`index.oidx`).
- **Custom Encoding**: Uses a language mapping (`lang.json`) for data encoding/decoding, enabling compact storage.
- **RESTful API**: Exposes POST and GET endpoints for storing and retrieving data via a Warp-based server.
- **Ethical Restrictions**: Prohibits use in malicious or unethical projects, enforced by the [DonutDB License v3](#license).
- **Open-Source**: Encourages contributions and source code sharing under a strict license to protect the project.

---

## Architecture

DonutDB is built with a modular architecture, leveraging Rust's performance and safety. Key components include:

- **API Layer** (`src/api.rs`): Handles HTTP requests using the Warp framework, providing POST (`/api/store`) and GET (`/api/store/<dataset>/<key>`) endpoints.
- **Crypto Layer** (`src/crypto.rs`): Implements AES-256 encryption/decryption using the `ring` crate, with keys derived from API keys.
- **Model Layer** (`src/model.rs`): Defines data structures for requests, schemas, metadata, and indexes using `serde` for serialization.
- **Utils Layer** (`src/utils.rs`): Provides helper functions for API key management, data encoding/decoding, and language mapping.
- **Storage Layer**: Manages columnar data storage, partitioning, and encrypted file operations.

Data is stored in a directory structure under `donutdb/<api_key>/`, with separate files for data, schemas, metadata, and indexes.

---

## File Structure

DonutDB stores data in a hierarchical, encrypted format:

donutdb/ └── <api_key>/ ├── meta.ometa          # Encrypted metadata (table name, record count, timestamps) ├── schema.oschema      # Encrypted schema (column names and types) ├── index.oidx          # Encrypted index for fast lookups └── data/ ├── .odb.part1  # Encrypted columnar data (e.g., id.odb.part1) ├── .odb.part2  # Additional partitions for large datasets

- **File Types**:
  - `.ometa`: Stores table metadata (e.g., record count, partition count, creation time).
  - `.oschema`: Defines the table schema (column names and types).
  - `.oidx`: Contains index entries for fast data retrieval.
  - `.odb.partX`: Stores encrypted columnar data, partitioned into 50MB chunks.

- **Encryption**: All files are encrypted using AES-256 with a key derived from the API key.

---

## Installation

### Prerequisites
- **Rust**: Install Rust via [rustup](https://rustup.rs/) (version 1.70 or later recommended).
- **Cargo**: Comes with Rust for dependency management.
- **Git**: For cloning the repository.

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/unknownmsv/DonutDB.git
   cd DonutDB

   ```
2. Create configuration files:
    api_keys.json:
    ```bash
    {"keys": ["test-key"]}
    ```
lang.json (for data encoding/decoding):
```bash
{"1": "01", "2": "02", "a": "alice", "b": "bob"}
```

3. Build the project:
```bash
cargo build --release
```
The server will start on http://localhost:4040.

### usage

The server listens on http://localhost:4040 and 
logs startup at the console.API EndpointsPOST /api/storeStores 
tabular data in a columnar format.Headers:Authorization: Bearer 
<api_key> (e.g., test-key)Content-Type: application/

jsonBody:
```json
{
  "dataset": "users",
  "slot": ["id", "username"],
  "data": [["1", "alice"], ["2", "bob"]]
}
```

example:

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

GET /api/store/<dataset>/<api_key>Retrieves stored data for a given dataset and API key.

example:

```bash
curl http://localhost:4040/api/store/users/test-key
```

LicenseDonutDB is licensed under the DonutDB License v3,
 a strict open-source license that requires:
 - Attribution to the original author (unknownmsv) and project (DonutDB).
 - Sharing source code for any distributed modifications.
 - Notification for commercial use.
 - See the LICENSE file for full details.

crator: @unknownmsv
discord: @unknownmsv