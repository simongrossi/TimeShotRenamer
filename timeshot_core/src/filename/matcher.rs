// timeshot_core/src/filename/matcher.rs

use chrono::NaiveDateTime;

// Fonction existante (garder)
pub fn is_date_in_filename(filename: &str, date: &Option<NaiveDateTime>) -> bool {
    if let Some(date) = date {
        let date_str = date.format("%Y-%m-%d").to_string();
        return filename.contains(&date_str);
    }
    false
}

// --- NOUVELLE FONCTION ---
/// Vérifie si le début du nom de fichier correspond au format YYYY-MM-DD_HHMMSS dérivé de la date EXIF.
pub fn filename_matches_exif_date(filename: &str, date_taken: &Option<NaiveDateTime>) -> bool {
    if let Some(date) = date_taken {
        // Génère le préfixe attendu à partir de la date EXIF
        let expected_prefix = date.format("%Y-%m-%d_%H%M%S").to_string();
        // Vérifie si le nom de fichier commence par ce préfixe
        // On peut aussi vérifier s'il est suivi par '_' ou '.' pour être plus strict,
        // mais commençons par start_with.
        filename.starts_with(&expected_prefix)
    } else {
        // Si pas de date EXIF, le nom ne peut pas correspondre
        false
    }
}
// --- FIN NOUVELLE FONCTION ---