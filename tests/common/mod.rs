use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

type Result<T> = anyhow::Result<T>;

pub fn compare_digests(output_file: &String, expected_file: &String) -> Result<bool> {
    let output_file_path = File::open(&output_file)?;
    let reader = BufReader::new(output_file_path);
    let output_file_digest = sha256_digest(reader)?;
    let output_digest = HEXUPPER.encode(output_file_digest.as_ref());

    let expected_file_path = File::open(&expected_file)?;
    let reader = BufReader::new(expected_file_path);
    let expected_file_digest = sha256_digest(reader)?;
    let expected_digest = HEXUPPER.encode(expected_file_digest.as_ref());

    return Ok(expected_digest == output_digest);
}

pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}
