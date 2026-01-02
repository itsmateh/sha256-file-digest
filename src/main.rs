use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use anyhow::Result;
use data_encoding::HEXUPPER;

mod cryptographic;

fn main() -> Result<()> {
    let path = "file_crypt.txt";

    let mut output = File::create(path)?;
    write!(output, "Nunca sucedió si no hay recuerdos de ello. La memoria humana es solo un registro. Solo necesitas reescribir ese registro.")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = cryptographic::sha256::sha256_digest(reader)?;

    println!("SHA-256 digest: {}",HEXUPPER.encode(digest.as_ref()));

    Ok(())
}
