
use std::collections::HashMap;
use crate::types::FileAnalysis;

pub fn generate_filename(
    analysis: &FileAnalysis,
    name_counter: &mut HashMap<String, usize>,
) -> String {
    let date = analysis.exif.date_taken
        .map(|d| d.format("%Y-%m-%d_%H%M%S").to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let base_name = analysis.original_name.replace(' ', "_");
    let folder = analysis.folder_name.replace(' ', "_");

    let key = format!("{date}");
    let count = name_counter.entry(key.clone()).or_insert(0);
    let suffix = if *count > 0 { format!("_{:02}", count) } else { "".to_string() };
    *count += 1;

    format!("{date}{suffix}_{folder}_{base_name}")
}
