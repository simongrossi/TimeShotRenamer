// timeshot_core/src/exif/analyze.rs

use std::path::{Path, PathBuf}; // Ajout de PathBuf
use walkdir::WalkDir;

// --- CORRECTION DE L'IMPORTATION ---
use crate::types::ExifData; // Chemin correct depuis la racine de la crate
// ---------------------------------

// Importez read_exif_data depuis le module reader du même niveau (exif)
use super::reader::read_exif_data;

// --- MODIFICATION DE LA SIGNATURE ET DU CORPS ---
pub fn analyze_directory(path: &Path) -> Vec<(PathBuf, ExifData)> {
    let mut results = Vec::new();
    println!("🔍 Analyse du répertoire : {}", path.display());

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    { // Accolade ouvrante de la boucle for
        let file_path = entry.path();
        println!("  Traitement de : {}", file_path.display());

        // Appelez read_exif_data qui devrait retourner ExifData.
        // Adaptez si elle retourne Option<ExifData> ou Result<ExifData>.
        let data: ExifData = read_exif_data(file_path);

        results.push((file_path.to_path_buf(), data));

    } // <-- *** ACCOLADE FERMANTE AJOUTÉE ICI ***

    println!("✅ Analyse terminée. {} fichiers traités.", results.len());
    results // Retournez le vecteur
}
// --- FIN DES MODIFICATIONS ---