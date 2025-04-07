// timeshot_core/src/lib.rs

use std::collections::HashMap;
use std::path::{Path, PathBuf}; // Gardons Path pour as_path()
use walkdir::WalkDir;
use crate::exif::reader::read_exif_data;
use crate::hash::compute::compute_file_hash;
use crate::hash::detect::mark_duplicates;
use crate::renamer::generator::generate_filename;
use crate::types::FileAnalysis;

pub mod types;
pub mod exif;
pub mod filename;
pub mod renamer;
pub mod hash;
pub mod export;

pub fn analyze_multiple_directories(dir_paths: Vec<PathBuf>, recursive: bool) -> Result<Vec<FileAnalysis>, String> {
    if dir_paths.is_empty() { return Ok(Vec::new()); }
    let mut valid_paths_found = false;
    for dir_path in &dir_paths { if dir_path.is_dir() { valid_paths_found = true; break; } }
    if !valid_paths_found { return Err("Aucun chemin de dossier valide fourni.".to_string()); }

    let mut analysis_results: Vec<FileAnalysis> = Vec::new();
    let mut name_counter: HashMap<String, usize> = HashMap::new();
    let mut errors: Vec<String> = Vec::new();
    let scan_type = if recursive { "récursive" } else { "simple" };
    println!("🔍 Lancement analyse {} sur {} répertoire(s)...", scan_type, dir_paths.len());

    for dir_path in dir_paths {
        if !dir_path.is_dir() { let error_msg = format!("Chemin fourni n'est pas un répertoire valide et sera ignoré : {}", dir_path.display()); eprintln!("Attention : {}", error_msg); errors.push(error_msg); continue; }
        println!("  -> Analyse de : {}", dir_path.display());
        let mut walker_builder = WalkDir::new(&dir_path).min_depth(1);
        if !recursive { walker_builder = walker_builder.max_depth(1); }
        for entry_result in walker_builder.into_iter() {
            match entry_result {
                Ok(entry) => { if entry.file_type().is_file() {
                    let file_path = entry.path();
                    let original_name = entry.file_name().to_string_lossy().to_string();
                    let parent_path = file_path.parent().unwrap_or(dir_path.as_path());
                    let current_folder_name = parent_path.file_name().map(|name| name.to_string_lossy().replace(' ', "_")).unwrap_or_else(|| "racine".to_string());
                    let exif_data = read_exif_data(file_path);
                    let file_hash = compute_file_hash(file_path);
                    let mut analysis = FileAnalysis { full_original_path: file_path.to_path_buf(), original_name: original_name.clone(), folder_name: current_folder_name, exif: exif_data, new_name: None, file_hash, is_duplicate: false };
                    analysis.new_name = Some(generate_filename(&analysis, &mut name_counter));
                    analysis_results.push(analysis); } }
                Err(e) => { let error_msg = format!("Erreur lecture entrée dans {}: {}", dir_path.display(), e); eprintln!("Attention : {}", error_msg); errors.push(error_msg); }
            }
        }
    }
    println!("🔍 Marquage des doublons sur l'ensemble des {} fichiers trouvés...", analysis_results.len());
    mark_duplicates(&mut analysis_results);
    println!("✅ Analyse {} terminée. {} fichiers traités au total.", scan_type, analysis_results.len());
    if !errors.is_empty() { println!("⚠️ {} erreurs rencontrées pendant l'analyse.", errors.len()); }
    Ok(analysis_results)
}