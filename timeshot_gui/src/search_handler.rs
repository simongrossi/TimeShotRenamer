// timeshot_gui/src/search_handler.rs

use crate::file_data_item::FileDataItem; // Pour FileDataItem::from_analysis
use gtk4::gio::ListStore;
use gtk4::glib::{self, clone, Object}; // Object pour downcast
use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Button, CheckButton, DialogFlags, MessageDialog, MessageType, ButtonsType,
    StringObject,
};
use std::path::PathBuf;
use timeshot_core::analyze_multiple_directories; // Import de la fonction Core

/// Connecte la logique d'analyse au clic du bouton "Chercher".
pub fn connect_search_button(
    search_button: &Button,
    directory_store: &ListStore, // Modèle contenant les StringObject des chemins
    results_model: &ListStore,   // Modèle ListStore<FileDataItem> pour les résultats
    recursive_checkbox: &CheckButton,
    window: &ApplicationWindow, // Fenêtre parente pour les dialogues
) {
    // Cloner les éléments nécessaires pour la closure du clic
    let results_model_search = results_model.clone();
    let directory_store_search = directory_store.clone();
    let recursive_checkbox_search = recursive_checkbox.clone();
    let window_clone_search = window.clone();

    search_button.connect_clicked(move |_| {
        println!("Bouton 'Chercher' cliqué (depuis search_handler)");
        results_model_search.remove_all(); // Vider les anciens résultats

        // 1. Collecter les chemins à scanner depuis le directory_store
        let mut paths_to_scan: Vec<PathBuf> = Vec::new();
        for i in 0..directory_store_search.n_items() {
            if let Some(obj) = directory_store_search.item(i) {
                // S'assurer que l'objet est bien un StringObject
                if let Ok(string_object) = obj.downcast::<StringObject>() {
                    paths_to_scan.push(PathBuf::from(string_object.string()));
                } else {
                    eprintln!("Erreur: L'objet dans directory_store n'est pas un StringObject valide à l'index {}", i);
                }
            } else {
                eprintln!("Erreur: Impossible de récupérer l'objet à l'index {} dans directory_store", i);
            }
        }

        // 2. Vérifier si des chemins ont été ajoutés
        if paths_to_scan.is_empty() {
            println!("Aucun répertoire à analyser.");
            let dialog = MessageDialog::new(Some(&window_clone_search), DialogFlags::MODAL, MessageType::Warning, ButtonsType::Ok, "Veuillez ajouter au moins un répertoire à analyser.");
            dialog.connect_response(|d, _| d.close());
            dialog.show();
            return; // Sortir si aucun dossier n'est listé
        }

        // 3. Lire l'option récursive
        let recursive = recursive_checkbox_search.is_active();
        println!("Analyse demandée pour {} répertoires. Récursif: {}", paths_to_scan.len(), recursive);

        // 4. Appeler la fonction Core analyze_multiple_directories
        // Note: On clone results_model_search encore une fois spécifiquement pour cette closure de résultat
        //       (même si ce n'est peut-être pas strictement nécessaire si elle n'est appelée qu'une fois)
        let results_model_clone_for_result = results_model_search.clone();
        match analyze_multiple_directories(paths_to_scan, recursive) {
            Ok(analysis_results) => {
                println!("Analyse terminée par le core, {} résultats reçus.", analysis_results.len());
                if analysis_results.is_empty() {
                    println!("Aucun fichier trouvé.");
                    // Optionnel: Afficher un message indiquant qu'aucun fichier n'a été trouvé
                }
                // Peupler le modèle de résultats
                for result in analysis_results {
                    let item = FileDataItem::from_analysis(&result);
                    results_model_clone_for_result.append(&item);
                }
            }
            Err(e) => {
                // Afficher une boîte de dialogue d'erreur
                eprintln!("Erreur globale lors de l'analyse des répertoires : {}", e);
                let dialog = MessageDialog::new(Some(&window_clone_search), DialogFlags::MODAL, MessageType::Error, ButtonsType::Ok, &format!("Erreur lors de l'analyse :\n{}", e));
                dialog.connect_response(|d, _| d.close());
                dialog.show();
            }
        }
        println!("Fin 'Chercher' (depuis search_handler)");
    });
}