// timeshot_core/src/lib.rs

// --- Imports nécessaires ---
use std::path::Path; // PathBuf retiré car non utilisé ici
use std::collections::HashMap;
use walkdir::WalkDir;
use crate::types::FileAnalysis;
use crate::exif::reader::read_exif_data;
use crate::hash::compute::compute_file_hash;
use crate::renamer::generator::generate_filename;
use crate::hash::detect::mark_duplicates;
// Pas besoin d'importer PathBuf ici car on l'utilise via file_path.to_path_buf()

// --- Déclarations de vos modules existants ---
pub mod types;
pub mod exif;
pub mod filename;
pub mod renamer;
pub mod hash;
pub mod export;

// --- Fonction d'analyse principale ---
/// Analyse un répertoire (récursivement ou non), collecte les infos, génère les noms, etc.
// ... (Le reste de la fonction analyze_and_prepare_files est identique à la version précédente que je vous ai donnée,
//      assurez-vous qu'elle contient bien le paramètre `recursive: bool` et la logique associée pour WalkDir,
//      et qu'elle remplit bien `full_original_path: file_path.to_path_buf()` dans FileAnalysis) ...
pub fn analyze_and_prepare_files(dir_path: &Path, recursive: bool) -> Result<Vec<FileAnalysis>, String> {
    if !dir_path.is_dir() {
        return Err(format!("Le chemin fourni n'est pas un répertoire : {}", dir_path.display()));
    }

    let mut analysis_results: Vec<FileAnalysis> = Vec::new();
    let mut name_counter: HashMap<String, usize> = HashMap::new();

    let scan_type = if recursive { "récursive" } else { "simple" };
    println!("🔍 Analyse {} du répertoire : {}", scan_type, dir_path.display());

    let mut walker_builder = WalkDir::new(dir_path).min_depth(1);
    if !recursive {
        walker_builder = walker_builder.max_depth(1);
    }

    for entry in walker_builder
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        let original_name = entry.file_name().to_string_lossy().to_string();
        let parent_path = file_path.parent().unwrap_or(dir_path);
        let current_folder_name = parent_path
            .file_name()
            .map(|name| name.to_string_lossy().replace(' ', "_"))
            .unwrap_or_else(|| "racine".to_string());

        let exif_data = read_exif_data(file_path);
        let file_hash = compute_file_hash(file_path);

        let mut analysis = FileAnalysis {
            // Remplir le champ ajouté dans types.rs
            full_original_path: file_path.to_path_buf(),
            original_name: original_name.clone(),
            folder_name: current_folder_name,
            exif: exif_data,
            new_name: None,
            file_hash,
            is_duplicate: false,
        };

        analysis.new_name = Some(generate_filename(&analysis, &mut name_counter));
        analysis_results.push(analysis);
    }

    println!("🔍 Marquage des doublons...");
    mark_duplicates(&mut analysis_results);

    println!("✅ Analyse {} terminée. {} fichiers traités.", scan_type, analysis_results.len());
    Ok(analysis_results)
}