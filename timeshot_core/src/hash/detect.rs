
use std::collections::HashMap;
use crate::types::FileAnalysis;

pub fn mark_duplicates(files: &mut [FileAnalysis]) {
    let mut hash_map: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, file) in files.iter().enumerate() {
        if let Some(hash) = &file.file_hash {
            hash_map.entry(hash.clone()).or_default().push(i);
        }
    }

    for indices in hash_map.values() {
        if indices.len() > 1 {
            for &i in indices {
                files[i].is_duplicate = true;
            }
        }
    }
}
