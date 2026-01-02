# Stream Hasher (SHA-256)

A lightweight CLI tool written in Rust to calculate the SHA-256 digest of a file. 

Unlike simple implementations that load the entire file into memory, this project uses **buffered streaming** to process files chunk by chunk. This allows it to verify the integrity of very large files (GBs or TBs) with minimal RAM usage.

## ⚡ Features

* **Memory Efficient:** Uses a fixed-size buffer (1KB) to read and update the hash context incrementally.
* **High Performance:** Built on top of [ring](https://github.com/briansmith/ring), a fast and safe crypto library.
* **Robust Error Handling:** Implements `anyhow` for clean error propagation.

## 📦 Dependencies

* `ring`: Safe, fast, small crypto using Rust.
* `anyhow`: Flexible key-value error handling.
* `data-encoding`: Hexadecimal encoding for the output.

## 🛠️ Implementation Details

The core logic resides in `src/cryptographic/sha256.rs`. It creates a `Context` and updates it in a loop as bytes are read from the file stream:

```rust
pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buff = [0; 1024];

    loop {
        let count = reader.read(&mut buff)?;
        if count == 0 { break; } // EOF
        context.update(&buff[..count]);
    }

    Ok(context.finish())
}