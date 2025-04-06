use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::NaiveDateTime;
use exif::{Tag, Reader};

#[derive(Debug, Clone)]
pub struct ImageFile {
    pub path: PathBuf,
    pub file_name: String,
    pub date_taken: Option<String>,
    pub selected: bool,
    pub preview_name: Option<String>,
    pub preview_valid: bool,
    pub date_in_name: bool,
    pub exif_date_matches_name: bool,
    pub exif_data: HashMap<String, String>,
}

pub fn collect_image_files(path: &Path, recursive: bool) -> Vec<ImageFile> {
    let mut files = Vec::new();

    let entries = if recursive {
        WalkDir::new(path)
    } else {
        WalkDir::new(path).max_depth(1)
    };

    for entry in entries
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_path_buf();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
        let exif_data = extract_exif_data(&path);

        let raw_date = exif_data.get("DateTimeOriginal").cloned();
        let parsed_date = raw_date.as_ref().and_then(|d| parse_date_flexible(d));
        let date_taken = parsed_date.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());

        let preview_name = Some(generate_preview_name(&file_name, &date_taken));

        let formats_to_test = parsed_date.map(|dt| {
            let y = dt.format("%Y").to_string();
            let m = dt.format("%m").to_string();
            let d = dt.format("%d").to_string();
            let h = dt.format("%H").to_string();
            let min = dt.format("%M").to_string();
            let s = dt.format("%S").to_string();

            vec![
                format!("{}-{}-{}_{}{}{}", y, m, d, h, min, s),
                format!("{}-{}-{}_{}-{}-{}", y, m, d, h, min, s),
                format!("{}_{}_{}_{}{}{}", y, m, d, h, min, s),
                format!("{}_{}_{}_{}-{}-{}", y, m, d, h, min, s),
                format!("{}.{}.{}_{}-{}-{}", y, m, d, h, min, s),
                format!("{}.{}.{}_{}{}{}", y, m, d, h, min, s),
                format!("{}{}{}_{}{}{}", y, m, d, h, min, s),
                format!("{}-{}-{}_{}{}{}", d, m, y, h, min, s),
                format!("{}-{}-{}_{}-{}-{}", d, m, y, h, min, s),
                format!("{}{}{}_{}{}{}", d, m, y, h, min, s)
            ]
        });

        let date_in_name = formats_to_test
            .as_ref()
            .map(|patterns| patterns.iter().any(|p| file_stem.contains(&p.to_lowercase())))
            .unwrap_or(false);

        let exif_date_matches_name = date_in_name;

        files.push(ImageFile {
            path,
            file_name,
            date_taken,
            selected: false,
            preview_name,
            preview_valid: true,
            date_in_name,
            exif_date_matches_name,
            exif_data,
        });
    }

    files
}

pub fn parse_date_flexible(input: &str) -> Option<NaiveDateTime> {
    let formats = vec![
        "%Y:%m:%d %H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%d-%m-%Y %H:%M:%S",
        "%Y.%m.%d %H:%M:%S",
        "%Y%m%d%H%M%S",
        "%Y%m%d_%H%M%S",
        "%Y_%m_%d_%H%M%S",
        "%Y.%m.%d_%H-%M-%S"
    ];

    for fmt in formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(input, fmt) {
            return Some(dt);
        }
    }
    None
}

fn extract_exif_data(path: &Path) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Ok(file) = fs::File::open(path) {
        let mut bufreader = std::io::BufReader::new(&file);
        if let Ok(reader) = Reader::new().read_from_container(&mut bufreader) {
            for f in reader.fields() {
                let tag_name = get_readable_tag_name(f.tag);
                let value = f.display_value().with_unit(&reader).to_string();
                let value = value.replace("\n", " ").replace("\r", " ");
                let trimmed = if value.len() > 100 {
                    format!("{}...", &value[..100])
                } else {
                    value
                };
                map.insert(tag_name, trimmed);
            }
        }
    }
    map
}

fn get_readable_tag_name(tag: Tag) -> String {
    format!("{}", tag)
}

fn generate_preview_name(original_name: &str, date_taken: &Option<String>) -> String {
    let base = Path::new(original_name).file_stem().unwrap_or_default().to_string_lossy();
    let ext = Path::new(original_name).extension().unwrap_or_default().to_string_lossy();

    let formatted_date = date_taken
        .as_ref()
        .and_then(|d| parse_date_flexible(d))
        .map(|dt| dt.format("%Y-%m-%d_%H%M%S").to_string())
        .unwrap_or_else(|| "unknown_date".to_string());

    format!("{}_{}.{}", formatted_date, base, ext)
}
