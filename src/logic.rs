// src/logic.rs
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct ImageFile {
    pub path: PathBuf,
    pub file_name: String,
    pub date_taken: Option<String>,
    pub exif_data: HashMap<String, String>,
    pub selected: bool,
    pub preview_name: Option<String>,
    pub preview_valid: bool,
    pub date_in_name: bool,
    pub exif_field_in_name: bool,
    pub error: Option<String>,
}

pub fn collect_image_files(path: &Path) -> Vec<ImageFile> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let entry_path = entry.path().to_path_buf();
        if !entry_path.is_file() {
            continue;
        }

        if let Some(ext) = entry_path.extension().and_then(|s| s.to_str()) {
            let ext = ext.to_lowercase();
            if !["jpg", "jpeg", "tif", "tiff"].contains(&ext.as_str()) {
                continue;
            }
        }

        let file_name = entry_path.file_name().unwrap().to_string_lossy().to_string();
        let (date_taken, exif_map, error) = read_exif_date(&entry_path);

        let date_in_name = if let Some(ref date) = date_taken {
            file_name_contains_date(&file_name, date)
        } else {
            false
        };

        let preview_name = Some(generate_preview_name(&file_name, &date_taken));

        files.push(ImageFile {
            path: entry_path,
            file_name,
            date_taken,
            exif_data: exif_map,
            selected: true,
            preview_name,
            preview_valid: true,
            date_in_name,
            exif_field_in_name: false,
            error,
        });
    }

    files
}

pub fn read_exif_date(path: &Path) -> (Option<String>, HashMap<String, String>, Option<String>) {
    let file = File::open(path);
    if file.is_err() {
        return (None, HashMap::new(), Some("Erreur ouverture fichier".into()));
    }

    let mut exif_data = HashMap::new();
    let mut reader = BufReader::new(file.unwrap());
    match exif::Reader::new().read_from_container(&mut reader) {
        Ok(exif) => {
            let mut date = None;
            for field in exif.fields() {
                let tag = format!("{}", field.tag);
                let value = field.display_value().to_string();
                if tag == "DateTimeOriginal" {
                    date = Some(value.clone());
                }
                exif_data.insert(tag, value);
            }
            (date, exif_data, None)
        }
        Err(_) => (None, HashMap::new(), Some("Erreur EXIF".into())),
    }
}

pub fn file_name_contains_date(file_name: &str, exif_date: &str) -> bool {
    let digits: String = exif_date.chars().filter(|c| c.is_ascii_digit()).collect();

    let cleaned_name: String = file_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    if digits.len() >= 8 {
        [digits.clone(), digits[..8].to_string()]
            .iter()
            .any(|p| cleaned_name.contains(p))
    } else {
        cleaned_name.contains(&digits)
    }
}

pub fn generate_preview_name(file_name: &str, date_taken: &Option<String>) -> String {
    if let Some(date) = date_taken {
        let formatted = date.replace(":", "-").replace(" ", "_");
        format!("{}_{}", formatted, file_name)
    } else {
        file_name.to_string()
    }
}
