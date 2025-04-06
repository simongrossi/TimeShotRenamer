
use crate::types::FileAnalysis;
use std::fs::File;

pub fn export_to_json(path: &str, data: &[FileAnalysis]) -> std::io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}
