
use crate::types::FileAnalysis;
use std::fs::File;
use csv::Writer;

pub fn export_to_csv(path: &str, data: &[FileAnalysis]) -> csv::Result<()> {
    let file = File::create(path)?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&[
        "original_name", "folder_name", "date_taken", "create_date", "modify_date",
        "artist", "title", "description", "keywords", "camera_model",
        "lens_model", "file_hash", "is_duplicate", "new_name"
    ])?;

    for f in data {
        writer.write_record(&[
            &f.original_name,
            &f.folder_name,
            &f.exif.date_taken.map(|d| d.to_string()).unwrap_or_default(),
            &f.exif.create_date.map(|d| d.to_string()).unwrap_or_default(),
            &f.exif.modify_date.map(|d| d.to_string()).unwrap_or_default(),
            f.exif.artist.as_deref().unwrap_or(""),
            f.exif.title.as_deref().unwrap_or(""),
            f.exif.description.as_deref().unwrap_or(""),
            &f.exif.keywords.join(","),
            f.exif.camera_model.as_deref().unwrap_or(""),
            f.exif.lens_model.as_deref().unwrap_or(""),
            f.file_hash.as_deref().unwrap_or(""),
            &f.is_duplicate.to_string(),
            f.new_name.as_deref().unwrap_or(""),
        ])?;
    }

    writer.flush()?;
    Ok(())
}
