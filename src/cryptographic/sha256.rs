use anyhow::Result; // manejo de errores
use ring::digest::{Context, Digest, SHA256}; // hashing
use data_encoding::HEXUPPER; // encoder hex
use std::fs::File;
use std::io::{BufReader, Read, Write};

// calcular hash SHA256 de una fuente de datos, sin cargar todo en memoria
pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest>{
    let mut context=Context::new(&SHA256);
    let mut buff = [0; 1024];

    loop{
        let count = reader.read(&mut buff)?; // leer hasta 1024 bytes
        if count == 0 { // eof
            break;
        }
        let valid_bytes = &buff[..count];
        context.update(valid_bytes); // actualizar hash
    }
    return Ok(context.finish());
}

