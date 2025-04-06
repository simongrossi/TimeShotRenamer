// timeshotrenamer_complet_final/timeshot_core/src/exif/reader.rs
use crate::types::ExifData;
use chrono::NaiveDateTime;
use exif::{Reader as KamadakReader, Tag, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// --- Fonction Principale ---

pub fn read_exif_data<P: AsRef<Path>>(path: P) -> ExifData {
    let mut result = ExifData {
        date_taken: None,
        create_date: None,
        modify_date: None,
        artist: None,
        title: None,
        description: None,
        keywords: Vec::new(),
        camera_model: None,
        lens_model: None,
        other_fields: HashMap::new(),
    };

    if let Ok(file) = File::open(path.as_ref()) {
        let mut buf_reader = BufReader::new(file);

        if let Ok(exif_data) = KamadakReader::new().read_from_container(&mut buf_reader) {
            for field in exif_data.fields() {
                match field.tag {
                    Tag::DateTimeOriginal => {
                        result.date_taken = parse_kamadak_date_value(&field.value)
                    }
                    Tag::DateTimeDigitized => {
                        result.create_date = parse_kamadak_date_value(&field.value)
                    }
                    Tag::DateTime => {
                        result.modify_date = parse_kamadak_date_value(&field.value)
                    }

                    Tag::Artist => result.artist = value_to_string(&field.value),
                    Tag::ImageDescription => result.description = value_to_string(&field.value),
                    Tag::Model => result.camera_model = value_to_string(&field.value),
                    Tag::LensModel => result.lens_model = value_to_string(&field.value),

                    _ => {
                        let tag_id = field.tag.number();
                        match tag_id {
                            0x9C9B => {
                                result.title = value_to_string(&field.value);
                            }
                            0x9C9E => {
                                result.keywords = parse_kamadak_keywords(&field.value);
                            }
                            _ => {
                                if let Some(val_str) = value_to_string(&field.value) {
                                    result.other_fields.insert(
                                        format!("{:?} (IFD{})", field.tag, field.ifd_num),
                                        val_str,
                                    );
                                } else {
                                    result.other_fields.insert(
                                        format!("{:?} (IFD{})", field.tag, field.ifd_num),
                                        format!("<Non-string value: {:?}>", field.value),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        } else {
            log::debug!("Could not read EXIF data from: {:?}", path.as_ref());
        }
    } else {
        log::warn!("Could not open file: {:?}", path.as_ref());
    }

    result
}

// --- Fonctions Helper (corrigées) ---

fn value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Ascii(ref vec) => {
            let strings: Vec<String> = vec
                .iter()
                .filter_map(|v| String::from_utf8(v.clone()).ok())
                .map(|s| s.trim().trim_end_matches('\0').to_string())
                .collect();
            if strings.is_empty() {
                None
            } else {
                Some(strings.join("; "))
            }
        }

        Value::Byte(ref bytes)
        | Value::Undefined(ref bytes, _) => String::from_utf8(bytes.clone())
            .ok()
            .map(|s| s.trim().trim_end_matches('\0').to_string()),

        Value::Short(ref nums) => Some(nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")),
        Value::Long(ref nums) => Some(nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")),
        Value::SLong(ref nums) => Some(nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")),
        Value::Float(ref nums) => Some(nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")),
        Value::Double(ref nums) => Some(nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")),

        Value::Rational(ref rats) => Some(rats.iter().map(|r| format!("{}/{}", r.num, r.denom)).collect::<Vec<_>>().join(", ")),
        Value::SRational(ref rats) => Some(rats.iter().map(|r| format!("{}/{}", r.num, r.denom)).collect::<Vec<_>>().join(", ")),

        _ => None,
    }
}

fn parse_kamadak_date_value(value: &Value) -> Option<NaiveDateTime> {
    match value {
        Value::Ascii(ref vec) if !vec.is_empty() => {
            if let Ok(s) = String::from_utf8(vec[0].clone()) {
                parse_exif_date_str(&s)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_exif_date_str(value: &str) -> Option<NaiveDateTime> {
    let trimmed = value.trim().trim_end_matches('\0');
    NaiveDateTime::parse_from_str(trimmed, "%Y:%m:%d %H:%M:%S").ok()
        .or_else(|| NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S").ok())
        .or_else(|| NaiveDateTime::parse_from_str(trimmed, "%Y:%m:%dT%H:%M:%S").ok())
}

fn parse_kamadak_keywords(value: &Value) -> Vec<String> {
    let mut keywords = Vec::new();
    match value {
        Value::Ascii(ref vec) => {
            for v in vec {
                if let Ok(s) = String::from_utf8(v.clone()) {
                    keywords.extend(
                        s.split(';')
                            .map(|k| k.trim().trim_end_matches('\0').to_string())
                            .filter(|k| !k.is_empty()),
                    );
                }
            }
        }
        Value::Undefined(ref bytes, _) => {
            if bytes.len() >= 2 && bytes.len() % 2 == 0 {
                let utf16_bytes: Vec<u16> = bytes
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();
                if let Ok(s) = String::from_utf16(&utf16_bytes) {
                    keywords.extend(
                        s.split(';')
                            .map(|k| k.trim().trim_end_matches('\0').to_string())
                            .filter(|k| !k.is_empty()),
                    );
                } else if let Ok(s) = String::from_utf8(bytes.clone()) {
                    keywords.extend(
                        s.split(';')
                            .map(|k| k.trim().trim_end_matches('\0').to_string())
                            .filter(|k| !k.is_empty()),
                    );
                } else {
                    log::warn!("Could not decode keywords bytes as UTF-16 or UTF-8");
                }
            } else if let Ok(s) = String::from_utf8(bytes.clone()) {
                keywords.extend(
                    s.split(';')
                        .map(|k| k.trim().trim_end_matches('\0').to_string())
                        .filter(|k| !k.is_empty()),
                );
            }
        }
        Value::Byte(ref bytes) => {
            if let Ok(s) = String::from_utf8(bytes.clone()) {
                keywords.extend(
                    s.split(';')
                        .map(|k| k.trim().trim_end_matches('\0').to_string())
                        .filter(|k| !k.is_empty()),
                );
            }
        }
        _ => {}
    }
    keywords.dedup();
    keywords
}
