
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn compute_file_hash(path: &Path) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();

    let mut buffer = [0u8; 8192];
    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    Some(hasher.finalize().to_hex().to_string())
}
