// timeshot_core/src/filename/mod.rs

pub mod parser;
pub mod matcher; // Garder ce module

// Optionnel: ré-exporter les fonctions directement si souhaité
// pub use parser::detect_date_pattern_in_filename;
// pub use matcher::{is_date_in_filename, filename_matches_exif_date};