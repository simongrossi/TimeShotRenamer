// timeshot_core/src/types.rs

use chrono::NaiveDateTime;
use std::collections::HashMap;
use serde::Serialize;
use std::path::PathBuf; // Gardé car utilisé dans FileAnalysis

// Correction de l'attribut derive
#[derive(Debug, Clone, Serialize)]
pub struct ExifData {
    pub date_taken: Option<NaiveDateTime>,
    pub create_date: Option<NaiveDateTime>,
    pub modify_date: Option<NaiveDateTime>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub other_fields: HashMap<String, String>,
}

// Correction de l'attribut derive
#[derive(Debug, Clone, Serialize)]
pub struct FileAnalysis {
    #[serde(skip)]
    pub full_original_path: PathBuf,
    pub original_name: String,
    pub folder_name: String,
    pub exif: ExifData,
    pub new_name: Option<String>,
    pub file_hash: Option<String>,
    pub is_duplicate: bool,
}