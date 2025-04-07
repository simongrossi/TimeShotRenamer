// timeshot_gui/src/ui.rs

use crate::file_data_item::FileDataItem;
use gtk4::gio::ListStore;
use gtk4::glib::{self, clone};
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, ColumnView,
    ColumnViewColumn, CssProvider, DialogFlags, Entry, Expander, FileChooserAction,
    FileChooserDialog, FilterListModel, Label, ListBox, ListItem, MessageDialog, MessageType,
    MultiSelection, Orientation, PolicyType, ResponseType, ScrolledWindow, SelectionMode,
    SignalListItemFactory, ButtonsType, StringObject, CustomFilter, FilterChange,
};
use gtk4::pango;
use once_cell::sync::Lazy; // <-- Import pour la Regex statique
use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::PathBuf,
    rc::Rc as StdRc,
    cell::RefCell as StdRefCell,
};

use timeshot_core::analyze_multiple_directories;

// --- Structure pour l'état des filtres ---
#[derive(Default)]
struct FilterState {
    excluded_extensions: HashSet<String>,
    filename_regex_str: String,
    filename_regex_enabled: bool,
    filename_regex: Option<Regex>,
    hide_already_named: bool,
    hide_if_name_has_date: bool, // <-- Nouvel état pour le filtre de date
}

// --- Regex compilée une seule fois pour vérifier la présence de date ---
static RE_DATE_IN_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:^|\b|[_.-])(\d{4}[-_]?\d{2}[-_]?\d{2})(?:$|\b|[_.-])")
        .expect("Regex pour date invalide")
});


// --- Fonctions Helper pour les Factories ---
fn create_label_factory(property_name: &'static str, is_markup: bool, css_classes: Vec<String>) -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_factory, list_item| {
        let label = Label::builder().halign(Align::Start).hexpand(true).wrap(true).wrap_mode(pango::WrapMode::WordChar).build();
        for css_class in &css_classes { label.add_css_class(css_class); }
        list_item.downcast_ref::<ListItem>().expect("Needs ListItem").set_child(Some(&label));
    });
    factory.connect_bind(move |_factory, list_item| {
        let list_item_gtk = list_item.downcast_ref::<ListItem>().expect("Needs ListItem");
        let item_option = list_item_gtk.item(); if item_option.is_none() { return; }
        let file_item = item_option.unwrap().downcast::<FileDataItem>().expect("Needs FileDataItem");
        let label_option = list_item_gtk.child(); if label_option.is_none() { return; }
        let label = label_option.unwrap().downcast::<Label>().expect("Needs Label");
        let value = file_item.property::<String>(property_name);
        let display_text = if value.is_empty() { "-" } else { &value };
        if is_markup { label.set_markup(display_text); } else { label.set_text(display_text); }
        if property_name == "original-name" { if file_item.property::<bool>("is-duplicate") { label.add_css_class("duplicate"); } else { label.remove_css_class("duplicate"); } }
    });
    factory
}
fn create_status_label_factory() -> SignalListItemFactory {
     let factory = SignalListItemFactory::new();
     factory.connect_setup(move |_factory, list_item| {
         let label = Label::builder().halign(Align::Start).build();
         list_item.downcast_ref::<ListItem>().expect("Needs ListItem").set_child(Some(&label));
     });
     factory.connect_bind(move |_factory, list_item| {
        let list_item_gtk = list_item.downcast_ref::<ListItem>().expect("Needs ListItem");
        let item_option = list_item_gtk.item(); if item_option.is_none() { return; }
        let file_item = item_option.unwrap().downcast::<FileDataItem>().expect("Needs FileDataItem");
        let label_option = list_item_gtk.child(); if label_option.is_none() { return; }
        let label = label_option.unwrap().downcast::<Label>().expect("Needs Label");
        if file_item.property::<bool>("is-duplicate") { label.set_text("Doublon"); label.add_css_class("duplicate"); } else { label.set_text(""); label.remove_css_class("duplicate"); }
     });
     factory
}
fn create_checkbox_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_factory, list_item| {
        let check = CheckButton::new();
        list_item.downcast_ref::<ListItem>().expect("Needs ListItem").set_child(Some(&check));
    });
    factory.connect_bind(move |_factory, list_item| {
        let list_item_gtk = list_item.downcast_ref::<ListItem>().expect("Needs ListItem");
        let item_option = list_item_gtk.item(); if item_option.is_none() { return; }
        let file_item = item_option.unwrap().downcast::<FileDataItem>().expect("Needs FileDataItem");
        let check_option = list_item_gtk.child(); if check_option.is_none() { return; }
        let check = check_option.unwrap().downcast::<CheckButton>().expect("Needs CheckButton");
        let is_selected_init = file_item.property::<bool>("selected"); check.set_active(is_selected_init);
        // Utiliser la nouvelle syntaxe clone recommandée à terme
        file_item.connect_notify_local(Some("selected"), clone!(@weak check => move |item, _pspec| {
            let is_selected = item.property::<bool>("selected");
            check.set_active(is_selected);
        }));
        // Utiliser la nouvelle syntaxe clone recommandée à terme
        check.connect_toggled(clone!(@weak file_item => move |check_button| {
            file_item.set_property("selected", check_button.is_active());
        }));
    });
    factory
}

// --- Fonction Principale de Construction UI ---
pub fn build_ui(app: &Application) {
    println!("DEBUG: build_ui() - Début");

    // --- CSS ---
    let provider = CssProvider::new();
    // Utiliser load_from_string("") à terme
    provider.load_from_data("label.duplicate { color: orange; font-style: italic; } columnview > header > button > label { font-weight: bold; } button.destructive-action { background-color: #e74c3c; color: white; }");
    gtk4::style_context_add_provider_for_display( &gtk4::gdk::Display::default().expect("Could not connect to a display."), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION );

    // --- Fenêtre principale ---
    let window = ApplicationWindow::builder().application(app).title("TimeShotRenamer GTK").default_width(1024).default_height(768).build();

    // --- Modèles de données ---
    let directory_store = ListStore::new::<StringObject>();
    let results_model = ListStore::new::<FileDataItem>();
    let filter_model = FilterListModel::new(Some(results_model.clone()), None::<gtk4::CustomFilter>);

    // --- Widgets ---

    // 1. Zone Gestion Répertoires
    let dir_vbox = GtkBox::new(Orientation::Vertical, 6);
    dir_vbox.set_margin_top(10); dir_vbox.set_margin_bottom(10);
    let dir_label = Label::builder().label("Répertoires à Analyser").halign(Align::Start).css_classes(vec!["heading".to_string()]).build();
    let directory_list_box = ListBox::new();
    directory_list_box.set_selection_mode(SelectionMode::Single);
    // Utiliser la nouvelle syntaxe clone recommandée à terme
    directory_store.connect_items_changed(clone!(@weak directory_list_box => move |store, _, _, _| {
        while let Some(child) = directory_list_box.first_child() { directory_list_box.remove(&child); }
        for i in 0..store.n_items() {
            if let Some(obj) = store.item(i) { if let Ok(string_object) = obj.downcast::<StringObject>() {
                let label = Label::new(Some(&string_object.string())); label.set_halign(Align::Start); directory_list_box.append(&label);
            }}}
    }));
    let dir_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never).vscrollbar_policy(PolicyType::Automatic)
        .min_content_height(100).max_content_height(200).child(&directory_list_box).build();
    let dir_button_hbox = GtkBox::new(Orientation::Horizontal, 6);
    let add_dir_button = Button::with_label("Ajouter");
    let remove_dir_button = Button::with_label("Retirer");
    let recursive_checkbox = CheckButton::with_label("Récursif"); recursive_checkbox.set_active(false);
    dir_button_hbox.append(&add_dir_button); dir_button_hbox.append(&remove_dir_button); dir_button_hbox.append(&recursive_checkbox);
    dir_vbox.append(&dir_label); dir_vbox.append(&dir_scrolled_window); dir_vbox.append(&dir_button_hbox);

    // 2. Bouton Chercher
    let search_button = Button::with_label("Chercher Fichiers");
    let search_button_hbox = GtkBox::new(Orientation::Horizontal, 0);
    search_button_hbox.set_halign(Align::Center); search_button_hbox.set_margin_top(5); search_button_hbox.set_margin_bottom(15);
    search_button_hbox.append(&search_button);

    // 3. Zone Résultats
    let results_vbox = GtkBox::new(Orientation::Vertical, 6);
    results_vbox.set_hexpand(true); results_vbox.set_vexpand(true); results_vbox.set_margin_bottom(10);

    // --- Zone de Filtres ---
    let filter_expander = Expander::new(Some("Filtres")); filter_expander.set_margin_bottom(10);
    let filter_grid = gtk4::Grid::builder().margin_start(10).margin_end(10).margin_top(5).margin_bottom(5).row_spacing(5).column_spacing(10).build();
    // Ligne 0: Extensions
    let filter_label_ext = Label::builder().label("Exclure extensions :").halign(Align::End).build();
    filter_grid.attach(&filter_label_ext, 0, 0, 1, 1);
    let excluded_extensions_entry = Entry::builder().placeholder_text("Ex: png, jpg").hexpand(true).build();
    filter_grid.attach(&excluded_extensions_entry, 1, 0, 1, 1);
    // Ligne 1: Regex Nom Fichier
    let filename_regex_check = CheckButton::builder().label("Regex Nom Fichier :").active(false).halign(Align::End).build();
    filter_grid.attach(&filename_regex_check, 0, 1, 1, 1);
    let filename_regex_entry = Entry::builder().placeholder_text("Expression régulière").sensitive(false).hexpand(true).build();
    filter_grid.attach(&filename_regex_entry, 1, 1, 1, 1);
    // Utiliser la nouvelle syntaxe clone recommandée à terme
    filename_regex_check.connect_toggled(clone!(@weak filename_regex_entry => move |check| {
        filename_regex_entry.set_sensitive(check.is_active());
    }));
    // Ligne 2: Masquer si nom proposé
    let hide_already_named_check = CheckButton::builder().label("Masquer si nom proposé").active(false).halign(Align::Start).build();
    filter_grid.attach(&hide_already_named_check, 1, 2, 1, 1);
    // Ligne 3: Masquer si nom contient date (NOUVEAU)
    let hide_if_name_has_date_check = CheckButton::builder()
        .label("Masquer si nom contient déjà AAAA-MM-JJ").tooltip_text("Masque les fichiers dont le nom original semble déjà contenir une date (YYYY-MM-DD, YYYY_MM_DD ou YYYYMMDD)")
        .active(false).halign(Align::Start).build();
    filter_grid.attach(&hide_if_name_has_date_check, 1, 3, 1, 1);
    filter_expander.set_child(Some(&filter_grid));

    // --- Création du ColumnView ---
    let results_selection_model = MultiSelection::new(Some(filter_model.clone()));
    let column_view = ColumnView::builder().model(&results_selection_model).show_column_separators(true).show_row_separators(true).build();
    // --- Définition des colonnes ---
    let check_factory = create_checkbox_factory(); let check_column = ColumnViewColumn::builder().title("✓").factory(&check_factory).fixed_width(40).resizable(false).build(); column_view.append_column(&check_column);
    let orig_factory = create_label_factory("original-name", false, vec![]); let orig_column = ColumnViewColumn::builder().title("Nom Original").factory(&orig_factory).expand(true).resizable(true).build(); column_view.append_column(&orig_column);
    let prop_factory = create_label_factory("proposed-name", false, vec![]); let prop_column = ColumnViewColumn::builder().title("Nom Proposé").factory(&prop_factory).expand(true).resizable(true).build(); column_view.append_column(&prop_column);
    let date_factory = create_label_factory("date-taken", false, vec![]); let date_column = ColumnViewColumn::builder().title("Date Prise").factory(&date_factory).fixed_width(160).resizable(true).build(); column_view.append_column(&date_column);
    let status_factory = create_status_label_factory(); let status_column = ColumnViewColumn::builder().title("Statut").factory(&status_factory).fixed_width(80).resizable(true).build(); column_view.append_column(&status_column);
    // --- ScrolledWindow pour ColumnView ---
    let results_scrolled_window = ScrolledWindow::builder().hscrollbar_policy(PolicyType::Automatic).vscrollbar_policy(PolicyType::Automatic).child(&column_view).vexpand(true).build();
    // --- Boutons d'action sous la liste ---
    let results_actions_hbox = GtkBox::new(Orientation::Horizontal, 6);
    let select_all_button = Button::with_label("Tout Sélectionner"); let deselect_all_button = Button::with_label("Tout Désélectionner"); let select_exif_button = Button::with_label("Sélectionner si Date EXIF");
    results_actions_hbox.append(&select_all_button); results_actions_hbox.append(&deselect_all_button); results_actions_hbox.append(&select_exif_button);
    results_actions_hbox.append(&GtkBox::builder().orientation(Orientation::Horizontal).hexpand(true).build()); // Spacer
    let rename_button = Button::with_label("Renommer Sélection"); rename_button.add_css_class("destructive-action"); results_actions_hbox.append(&rename_button);
    // --- Assemblage Zone Résultats ---
    results_vbox.append(&filter_expander); results_vbox.append(&results_scrolled_window); results_vbox.append(&results_actions_hbox);

    // --- Assemblage Final UI ---
    let root_vbox = GtkBox::new(Orientation::Vertical, 6);
    root_vbox.set_margin_top(10); root_vbox.set_margin_bottom(10); root_vbox.set_margin_start(10); root_vbox.set_margin_end(10);
    root_vbox.append(&dir_vbox); root_vbox.append(&search_button_hbox); root_vbox.append(&results_vbox);
    window.set_child(Some(&root_vbox));

    // ************************************************************************* //
    // ***** DEBUT : Logique des boutons et filtres (partie qui manquait) ***** //
    // ************************************************************************* //

    // --- Logique Bouton "Ajouter Répertoire" ---
    let window_clone_add = window.clone();
    let directory_store_add = directory_store.clone();
    add_dir_button.connect_clicked(move |_| {
        // Utiliser FileChooserNative à terme
        let dialog = FileChooserDialog::new( Some("Ajouter un Répertoire"), Some(&window_clone_add), FileChooserAction::SelectFolder, &[("Annuler", ResponseType::Cancel), ("Ajouter", ResponseType::Accept)], );
        let store_clone = directory_store_add.clone();
        // Utiliser les méthodes async/await avec FileChooserNative à terme
        dialog.connect_response(move |d, response| {
            if response == ResponseType::Accept {
                if let Some(file) = d.file() { // d.file() déprécié
                    if let Some(path) = file.path() {
                        if let Some(path_str) = path.to_str() {
                            store_clone.append(&StringObject::new(path_str));
                            println!("Répertoire ajouté : {}", path_str);
                        }
                    }
                }
            }
            d.close(); // d.close() déprécié
        });
        dialog.show(); // dialog.show() déprécié
    });

    // --- Logique Bouton "Retirer Répertoire" ---
    let directory_store_remove = directory_store.clone();
    let directory_list_box_remove = directory_list_box.clone();
    remove_dir_button.connect_clicked(move |_| {
        if let Some(selected_row) = directory_list_box_remove.selected_row() {
            let index = selected_row.index();
            if index >= 0 {
                directory_store_remove.remove(index as u32);
                println!("Répertoire retiré à l'index {}", index);
            }
        } else {
            println!("Aucun répertoire sélectionné pour le retrait.");
        }
    });

    // --- Logique Bouton "Chercher" ---
    let results_model_search = results_model.clone();
    let directory_store_search = directory_store.clone();
    let recursive_checkbox_search = recursive_checkbox.clone();
    let window_clone_search = window.clone();
    search_button.connect_clicked(move |_| {
        println!("Bouton 'Chercher Fichiers' cliqué");
        results_model_search.remove_all();
        let mut paths_to_scan: Vec<PathBuf> = Vec::new();
        for i in 0..directory_store_search.n_items() {
            if let Some(obj) = directory_store_search.item(i) {
                if let Ok(string_object) = obj.downcast::<StringObject>() {
                    paths_to_scan.push(PathBuf::from(string_object.string()));
                }
            }
        }
        if paths_to_scan.is_empty() {
            // Utiliser AlertDialog à terme
            let dialog = MessageDialog::new(Some(&window_clone_search), DialogFlags::MODAL, MessageType::Warning, ButtonsType::Ok, "Veuillez ajouter au moins un répertoire à analyser.");
            dialog.connect_response(|d, _| d.close()); // Déprécié
            dialog.show(); // Déprécié
            return;
        }
        let recursive = recursive_checkbox_search.is_active();
        println!("Analyse demandée pour {} répertoires. Récursif: {}", paths_to_scan.len(), recursive);
        let results_model_clone = results_model_search.clone();
        match analyze_multiple_directories(paths_to_scan, recursive) {
            Ok(analysis_results) => {
                println!("Analyse terminée par le core, {} résultats reçus.", analysis_results.len());
                if analysis_results.is_empty() { println!("Aucun fichier trouvé."); }
                for result in analysis_results {
                    let item = FileDataItem::from_analysis(&result);
                    results_model_clone.append(&item);
                }
            }
            Err(e) => {
                eprintln!("Erreur globale lors de l'analyse des répertoires : {}", e);
                // Utiliser AlertDialog à terme
                let dialog = MessageDialog::new(Some(&window_clone_search), DialogFlags::MODAL, MessageType::Error, ButtonsType::Ok, &format!("Erreur lors de l'analyse :\n{}", e));
                dialog.connect_response(|d, _| d.close()); // Déprécié
                dialog.show(); // Déprécié
            }
        }
        println!("Fin 'Chercher Fichiers'");
     });

    // --- Logique Boutons Sélection ---
    let model_select_all = results_model.clone();
    select_all_button.connect_clicked(move |_| {
        println!("Bouton 'Tout Sélectionner' cliqué");
        for i in 0..model_select_all.n_items() {
            if let Some(obj) = model_select_all.item(i) {
                if let Ok(item) = obj.downcast::<FileDataItem>() { item.set_property("selected", true); }
            }
        }
        println!("Fin 'Tout Sélectionner'");
    });
    let model_deselect_all = results_model.clone();
    deselect_all_button.connect_clicked(move |_| {
        println!("Bouton 'Tout Désélectionner' cliqué");
        for i in 0..model_deselect_all.n_items() {
            if let Some(obj) = model_deselect_all.item(i) {
                if let Ok(item) = obj.downcast::<FileDataItem>() { item.set_property("selected", false); }
            }
        }
        println!("Fin 'Tout Désélectionner'");
    });
    let model_select_exif = results_model.clone();
    select_exif_button.connect_clicked(move |_| {
        println!("Bouton 'Sélectionner si Date EXIF' cliqué");
        for i in 0..model_select_exif.n_items() {
            if let Some(obj) = model_select_exif.item(i) {
                if let Ok(item) = obj.downcast::<FileDataItem>() {
                    let date_str = item.property::<String>("date-taken");
                    let has_exif_date = !date_str.is_empty() && date_str != "-";
                    item.set_property("selected", has_exif_date);
                }
            }
        }
        println!("Fin 'Sélectionner si Date EXIF'");
    });

    // --- Logique Bouton Renommer ---
    let model_rename = results_model.clone();
    let window_clone_rename = window.clone();
    rename_button.connect_clicked(move |_| {
        println!("Bouton 'Renommer Sélection' cliqué");
        let mut items_to_rename: Vec<(PathBuf, PathBuf, u32)> = Vec::new();
        let mut errors: Vec<String> = Vec::new(); let mut success_count = 0; let mut skipped_count = 0;
        for i in 0..model_rename.n_items() {
            if let Some(obj) = model_rename.item(i) {
                if let Ok(item) = obj.downcast::<FileDataItem>() {
                    if item.property::<bool>("selected") {
                        let original_path = item.full_original_path();
                        let proposed_name = item.property::<String>("proposed-name");
                        if !proposed_name.is_empty() && proposed_name != "-" {
                            if let Some(parent_dir) = original_path.parent() {
                                let new_path = parent_dir.join(&proposed_name);
                                if original_path != new_path { items_to_rename.push((original_path.clone(), new_path, i)); }
                                else { skipped_count += 1; }
                            } else { errors.push(format!("Pas de parent pour: {}", original_path.display())); }
                        } else { skipped_count += 1; }
                    }
                }
            }
        }
        if items_to_rename.is_empty() && errors.is_empty() {
            // Utiliser AlertDialog à terme
            let dialog = MessageDialog::new( Some(&window_clone_rename), DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT, MessageType::Info, ButtonsType::Ok, "Aucun fichier valide sélectionné pour le renommage.");
            dialog.connect_response(|d, _| d.close()); dialog.show(); // Déprécié
            return;
        }
        println!("Tentative de renommage de {} fichier(s)...", items_to_rename.len());
        let mut indices_to_remove = Vec::new();
        // Itérer en ordre inverse pour que les indices restent valides après suppression
        for item_info in items_to_rename.iter().rev() {
            let original_path = &item_info.0; let new_path = &item_info.1; let model_index = item_info.2;
            println!("Renommage de {} -> {}", original_path.display(), new_path.display());
            match fs::rename(original_path, new_path) {
                Ok(_) => { success_count += 1; indices_to_remove.push(model_index); }
                Err(e) => { let error_msg = format!("Erreur renommage '{}': {}", original_path.display(), e); eprintln!("{}", error_msg); errors.push(error_msg); }
            }
        }
        // Supprimer les éléments renommés du modèle (ordre inverse déjà géré par l'itération précédente)
        indices_to_remove.sort_unstable(); // Trier les indices pour la suppression
        indices_to_remove.reverse(); // Supprimer de la fin vers le début
        for index in &indices_to_remove { model_rename.remove(*index); }

        // Afficher le résumé
        let mut summary = format!("Renommage terminé.\n\nSuccès : {}\nÉchecs : {}\nSkippés : {}\n", success_count, errors.len(), skipped_count);
        if !errors.is_empty() {
            summary.push_str("\nDétails des erreurs :\n");
            for err in errors.iter().take(10) { summary.push_str(&format!("- {}\n", err)); }
            if errors.len() > 10 { summary.push_str("...\n"); }
        }
        // Utiliser AlertDialog à terme
        let dialog = MessageDialog::new( Some(&window_clone_rename), DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT, if errors.is_empty() { MessageType::Info } else { MessageType::Warning }, ButtonsType::Ok, &summary);
        dialog.connect_response(|d, _| d.close()); dialog.show(); // Déprécié
        println!("Fin 'Renommer Sélection'");
     });

    // --- Logique de Filtrage ---
    let filter_state = StdRc::new(StdRefCell::new(FilterState::default()));

    // --- Définition du filtre personnalisé ---
    let filter_state_clone = filter_state.clone();
    let custom_filter = CustomFilter::new(move |obj| {
        let filter_state = filter_state_clone.borrow();
        if let Ok(file_item) = obj.clone().downcast::<FileDataItem>() {
            let original_name = file_item.property::<String>("original-name");

            // 1. Ext.
            if !filter_state.excluded_extensions.is_empty() {
                if let Some(dot_pos) = original_name.rfind('.') {
                    let extension = &original_name[dot_pos + 1..].to_lowercase();
                    if filter_state.excluded_extensions.contains(extension) { return false; }
                }
            }
            // 2. Regex
            if filter_state.filename_regex_enabled {
                if let Some(regex) = &filter_state.filename_regex {
                    if !regex.is_match(&original_name) { return false; }
                }
            }
            // 3. Nom proposé
            if filter_state.hide_already_named {
                let proposed = file_item.property::<String>("proposed-name");
                if !proposed.is_empty() && proposed != "-" { return false; }
            }
            // 4. Date dans nom (NOUVEAU)
            if filter_state.hide_if_name_has_date {
                if RE_DATE_IN_NAME.is_match(&original_name) { return false; }
            }
            true // Montrer si pas filtré
        } else { true } // Montrer si pas FileDataItem
    });
    filter_model.set_filter(Some(&custom_filter));


    // --- Closure pour mettre à jour l'état et déclencher le refiltrage ---
    let update_filter = {
        let filter_state = filter_state.clone();
        let excluded_extensions_entry = excluded_extensions_entry.clone();
        let filename_regex_check = filename_regex_check.clone();
        let filename_regex_entry = filename_regex_entry.clone();
        let hide_already_named_check = hide_already_named_check.clone();
        let hide_if_name_has_date_check = hide_if_name_has_date_check.clone(); // <-- Clone new checkbox
        let custom_filter_clone = custom_filter.clone();

        move || {
            println!("Mise à jour du filtre demandée...");
            {   // Bloc d'emprunt mutable
                let mut state = filter_state.borrow_mut();
                state.excluded_extensions = excluded_extensions_entry.text().split(',')
                    .map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect();
                state.filename_regex_enabled = filename_regex_check.is_active();
                state.filename_regex_str = filename_regex_entry.text().to_string();
                state.filename_regex = if state.filename_regex_enabled && !state.filename_regex_str.is_empty() {
                    match Regex::new(&state.filename_regex_str) { Ok(re) => Some(re), Err(e) => { eprintln!("Erreur Regex: {}", e); None } }
                } else { None };
                state.hide_already_named = hide_already_named_check.is_active();
                state.hide_if_name_has_date = hide_if_name_has_date_check.is_active(); // <-- MàJ nouvel état
                //println!("  Debug FilterState: {:?}", state); // Décommenter pour debug complet de l'état
            } // Fin emprunt mutable
            custom_filter_clone.changed(FilterChange::Different); // Déclencher le refiltrage
            println!("Refiltrage déclenché.");
        }
    };

    // --- Connexions de signaux pour les filtres ---
    excluded_extensions_entry.connect_changed({ let update_filter = update_filter.clone(); move |_| { update_filter(); } });
    filename_regex_entry.connect_changed({ let update_filter = update_filter.clone(); move |_| { update_filter(); } });
    filename_regex_check.connect_toggled({ let update_filter = update_filter.clone(); move |_| { update_filter(); } });
    hide_already_named_check.connect_toggled({ let update_filter = update_filter.clone(); move |_| { update_filter(); } });
    // Connexion pour la nouvelle case (correction: pas besoin de clone si c'est la dernière)
    hide_if_name_has_date_check.connect_toggled(move |_| { update_filter(); });


    // *********************************************************************** //
    // ***** FIN : Logique des boutons et filtres (partie qui manquait) ***** //
    // *********************************************************************** //

    println!("DEBUG: build_ui() - Avant window.present()");
    window.present(); // Déprécié
    println!("DEBUG: build_ui() - Après window.present()");
} // Fin de build_ui