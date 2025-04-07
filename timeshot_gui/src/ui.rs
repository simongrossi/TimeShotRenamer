// timeshot_gui/src/ui.rs

// Imports nécessaires (incluant celui pour le nouveau module)
use crate::file_data_item::FileDataItem;
use crate::search_handler; // <-- Ajout pour le nouveau module
use gtk4::gio::ListStore;
use gtk4::glib::{self, clone, Object};
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, ColumnView,
    ColumnViewColumn, CssProvider, DialogFlags, Entry, Expander, FileChooserAction,
    FileChooserDialog, FilterListModel, Label, ListBox, ListItem, MessageDialog, MessageType,
    MultiSelection, Orientation, PolicyType, ResponseType, ScrolledWindow, SelectionMode,
    SignalListItemFactory, ButtonsType, StringObject, CustomFilter, FilterChange, Filter,
};
use gtk4::pango;
use regex::Regex;
use std::{
    collections::HashSet,
    fs, // Gardé pour le bouton Renommer
    path::PathBuf, // Gardé pour le bouton Renommer
    rc::Rc as StdRc,
    cell::RefCell as StdRefCell,
};

// Note: analyze_multiple_directories n'est plus appelé directement ici
// use timeshot_core::analyze_multiple_directories;

// --- Structure FilterState (inchangée) ---
#[derive(Default)]
struct FilterState {
    excluded_extensions: HashSet<String>,
    filename_regex_str: String,
    filename_regex_enabled: bool,
    filename_regex: Option<Regex>,
}

// --- Fonctions Helper pour les Factories (inchangées) ---
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
        file_item.connect_notify_local(Some("selected"), clone!(@weak check => move |item, _pspec| {
            let is_selected = item.property::<bool>("selected");
            check.set_active(is_selected);
        }));
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
    provider.load_from_data("label.duplicate { color: orange; font-style: italic; } columnview > header > button > label { font-weight: bold; } button.destructive-action { background-color: #e74c3c; color: white; }");
    gtk4::style_context_add_provider_for_display( &gtk4::gdk::Display::default().expect("Could not connect to a display."), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION );

    // --- Fenêtre principale ---
    let window = ApplicationWindow::builder().application(app).title("TimeShotRenamer GTK (Filtres)").default_width(1280).default_height(750).build();

    // --- Modèles de données ---
    let directory_store = ListStore::new::<StringObject>();
    let results_model = ListStore::new::<FileDataItem>();
    let filter_model = FilterListModel::new(Some(results_model.clone()), None::<gtk4::Filter>);

    // --- Widgets Principaux ---

    // 1. Zone Gestion Répertoires
    let dir_vbox = GtkBox::new(Orientation::Vertical, 6);
    dir_vbox.set_margin_top(10); dir_vbox.set_margin_bottom(10); dir_vbox.set_margin_start(10); dir_vbox.set_margin_end(10);
    let dir_label = Label::builder().label("Répertoires à Analyser").halign(Align::Start).css_classes(vec!["heading".to_string()]).build();
    let directory_list_box = ListBox::new();
    directory_list_box.set_selection_mode(SelectionMode::Single);
    directory_store.connect_items_changed(clone!(@weak directory_list_box => move |store, _, _, _| { // Closure complète
        while let Some(child) = directory_list_box.first_child() { directory_list_box.remove(&child); }
        for i in 0..store.n_items() { if let Some(obj) = store.item(i) { if let Ok(string_object) = obj.downcast::<StringObject>() { let label = Label::new(Some(&string_object.string())); label.set_halign(Align::Start); directory_list_box.append(&label); } } }
    }));
    let dir_scrolled_window = ScrolledWindow::builder().hscrollbar_policy(PolicyType::Never).min_content_height(150).child(&directory_list_box).build();
    let dir_button_hbox = GtkBox::new(Orientation::Horizontal, 6);
    let add_dir_button = Button::with_label("Ajouter");
    let remove_dir_button = Button::with_label("Retirer");
    let recursive_checkbox = CheckButton::with_label("Récursif"); recursive_checkbox.set_active(false);
    dir_button_hbox.append(&add_dir_button); dir_button_hbox.append(&remove_dir_button); dir_button_hbox.append(&recursive_checkbox);
    dir_vbox.append(&dir_label); dir_vbox.append(&dir_scrolled_window); dir_vbox.append(&dir_button_hbox);

    // 2. Zone Résultats
    let results_vbox = GtkBox::new(Orientation::Vertical, 6);
     results_vbox.set_hexpand(true); results_vbox.set_vexpand(true); results_vbox.set_margin_top(10); results_vbox.set_margin_bottom(10); results_vbox.set_margin_start(10); results_vbox.set_margin_end(10);

    // --- Zone de Filtres ---
    let filter_expander = Expander::new(Some("Filtres"));
    filter_expander.set_margin_bottom(10);
    let filter_grid = gtk4::Grid::builder().margin_start(10).margin_end(10).margin_top(5).margin_bottom(5).row_spacing(5).column_spacing(10).build();
    let filter_label_ext = Label::builder().label("Exclure extensions :").halign(Align::End).build();
    filter_grid.attach(&filter_label_ext, 0, 0, 1, 1);
    let excluded_extensions_entry = Entry::builder().placeholder_text("Ex: png, jpg, tif (séparées par virgule)").hexpand(true).build();
    filter_grid.attach(&excluded_extensions_entry, 1, 0, 2, 1);
    let filename_regex_check = CheckButton::builder().label("Regex Nom Fichier :").active(false).halign(Align::End).build();
    filter_grid.attach(&filename_regex_check, 0, 1, 1, 1);
    let filename_regex_entry = Entry::builder().placeholder_text("Expression régulière").sensitive(false).hexpand(true).build();
    filter_grid.attach(&filename_regex_entry, 1, 1, 1, 1);
    filename_regex_check.connect_toggled(clone!(@weak filename_regex_entry => move |check| { // Closure complète
        filename_regex_entry.set_sensitive(check.is_active());
    }));
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
     let select_all_button = Button::with_label("Tout Sélectionner");
     let deselect_all_button = Button::with_label("Tout Désélectionner");
     let select_exif_button = Button::with_label("Sélectionner si Date EXIF");
     results_actions_hbox.append(&select_all_button); results_actions_hbox.append(&deselect_all_button); results_actions_hbox.append(&select_exif_button);
     results_actions_hbox.append(&GtkBox::builder().orientation(Orientation::Horizontal).hexpand(true).build());
     let rename_button = Button::with_label("Renommer Sélection"); rename_button.add_css_class("destructive-action");
     results_actions_hbox.append(&rename_button);

    // --- Assemblage Zone Résultats ---
    results_vbox.append(&filter_expander); results_vbox.append(&results_scrolled_window); results_vbox.append(&results_actions_hbox);

    // 3. Bouton Chercher
    let search_button = Button::with_label("Chercher");
     search_button.set_margin_top(10); search_button.set_margin_bottom(10);

    // --- Assemblage Final UI ---
    let main_paned = gtk4::Paned::new(Orientation::Horizontal);
    main_paned.set_start_child(Some(&dir_vbox)); main_paned.set_end_child(Some(&results_vbox));
    main_paned.set_position(300); main_paned.set_wide_handle(true);
    let top_control_hbox = GtkBox::new(Orientation::Horizontal, 6); top_control_hbox.set_margin_bottom(10); top_control_hbox.append(&search_button);
    let root_vbox = GtkBox::new(Orientation::Vertical, 0); root_vbox.append(&top_control_hbox); root_vbox.append(&main_paned);
    window.set_child(Some(&root_vbox));


    // --- Logique Bouton "Ajouter Répertoire" (Complète) ---
    let window_clone_add = window.clone();
    let directory_store_add = directory_store.clone();
    add_dir_button.connect_clicked(move |_| { // Closure complète
         let dialog = FileChooserDialog::new( Some("Ajouter un Répertoire"), Some(&window_clone_add), FileChooserAction::SelectFolder, &[("Annuler", ResponseType::Cancel), ("Ajouter", ResponseType::Accept)], );
        let store_clone = directory_store_add.clone();
        dialog.connect_response(move |d, response| { if response == ResponseType::Accept { if let Some(file) = d.file() { if let Some(path) = file.path() { if let Some(path_str) = path.to_str() { store_clone.append(&StringObject::new(path_str)); println!("Répertoire ajouté : {}", path_str); } } } } d.close(); }); dialog.show();
    });

    // --- Logique Bouton "Retirer Répertoire" (Complète) ---
    let directory_store_remove = directory_store.clone();
    let directory_list_box_remove = directory_list_box.clone();
    remove_dir_button.connect_clicked(move |_| { // Closure complète
        if let Some(selected_row) = directory_list_box_remove.selected_row() { let index = selected_row.index(); if index >= 0 { directory_store_remove.remove(index as u32); println!("Répertoire retiré à l'index {}", index); } } else { println!("Aucun répertoire sélectionné pour le retrait."); }
    });

    // --- ***** MODIFIÉ : Appel au nouveau module pour la logique "Chercher" ***** ---
    search_handler::connect_search_button(
        &search_button,
        &directory_store, // Passer le store des répertoires
        &results_model,   // Passer le store SOURCE des résultats
        &recursive_checkbox,
        &window           // Passer la fenêtre parente
    );
    // --- Fin Modification ---

    // --- Logique Boutons Sélection (Complètes) ---
    let model_select_all = results_model.clone(); // Opère toujours sur results_model
    select_all_button.connect_clicked(move |_| { // Closure complète
        println!("Bouton 'Tout Sélectionner' cliqué"); for i in 0..model_select_all.n_items() { if let Some(obj) = model_select_all.item(i) { if let Ok(item) = obj.downcast::<FileDataItem>() { item.set_property("selected", true); } } } println!("Fin 'Tout Sélectionner'");
    });
    let model_deselect_all = results_model.clone(); // Opère toujours sur results_model
    deselect_all_button.connect_clicked(move |_| { // Closure complète
        println!("Bouton 'Tout Désélectionner' cliqué"); for i in 0..model_deselect_all.n_items() { if let Some(obj) = model_deselect_all.item(i) { if let Ok(item) = obj.downcast::<FileDataItem>() { item.set_property("selected", false); } } } println!("Fin 'Tout Désélectionner'");
    });
    let model_select_exif = results_model.clone(); // Opère toujours sur results_model
    select_exif_button.connect_clicked(move |_| { // Closure complète
        println!("Bouton 'Sélectionner si Date EXIF' cliqué"); for i in 0..model_select_exif.n_items() { if let Some(obj) = model_select_exif.item(i) { if let Ok(item) = obj.downcast::<FileDataItem>() { let date_str = item.property::<String>("date-taken"); let has_exif_date = !date_str.is_empty() && date_str != "-"; item.set_property("selected", has_exif_date); } } } println!("Fin 'Sélectionner si Date EXIF'");
    });

    // --- Logique Bouton Renommer (Complète) ---
    let model_rename = results_model.clone(); // Opère toujours sur results_model
    let window_clone_rename = window.clone();
    rename_button.connect_clicked(move |_| { // Closure complète
        println!("Bouton 'Renommer Sélection' cliqué"); let mut items_to_rename: Vec<(PathBuf, PathBuf, u32)> = Vec::new(); let mut errors: Vec<String> = Vec::new(); let mut success_count = 0; let mut skipped_count = 0; for i in 0..model_rename.n_items() { if let Some(obj) = model_rename.item(i) { if let Ok(item) = obj.downcast::<FileDataItem>() { if item.property::<bool>("selected") { let original_path = item.full_original_path(); let proposed_name = item.property::<String>("proposed-name"); if !proposed_name.is_empty() && proposed_name != "-" { if let Some(parent_dir) = original_path.parent() { let new_path = parent_dir.join(&proposed_name); if original_path != new_path { items_to_rename.push((original_path.clone(), new_path, i)); } else { skipped_count += 1; } } else { errors.push(format!("Pas de parent pour: {}", original_path.display())); } } else { skipped_count += 1; } } } } } if items_to_rename.is_empty() && errors.is_empty() { let dialog = MessageDialog::new( Some(&window_clone_rename), DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT, MessageType::Info, ButtonsType::Ok, "Aucun fichier valide sélectionné pour le renommage."); dialog.connect_response(|d, _| d.close()); dialog.show(); return; } println!("Tentative de renommage de {} fichier(s)...", items_to_rename.len()); let mut indices_to_remove = Vec::new(); for item_info in items_to_rename.iter().rev() { let original_path = &item_info.0; let new_path = &item_info.1; let model_index = item_info.2; println!("Renommage de {} -> {}", original_path.display(), new_path.display()); match fs::rename(original_path, new_path) { Ok(_) => { success_count += 1; indices_to_remove.push(model_index); } Err(e) => { let error_msg = format!("Erreur renommage '{}': {}", original_path.display(), e); eprintln!("{}", error_msg); errors.push(error_msg); } } } indices_to_remove.sort_unstable(); indices_to_remove.reverse(); for index in &indices_to_remove { model_rename.remove(*index); } let mut summary = format!("Renommage terminé.\n\nSuccès : {}\nÉchecs : {}\nSkippés : {}\n", success_count, errors.len(), skipped_count); if !errors.is_empty() { summary.push_str("\nDétails des erreurs :\n"); for err in errors.iter().take(10) { summary.push_str(&format!("- {}\n", err)); } if errors.len() > 10 { summary.push_str("...\n"); } } let dialog = MessageDialog::new( Some(&window_clone_rename), DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT, if errors.is_empty() { MessageType::Info } else { MessageType::Warning }, ButtonsType::Ok, &summary); dialog.connect_response(|d, _| d.close()); dialog.show(); println!("Fin 'Renommer Sélection'");
     });

    // --- Logique de Filtrage (Complète) ---
    let filter_state = StdRc::new(StdRefCell::new(FilterState::default()));
    let filter_state_clone = filter_state.clone();
    let custom_filter = CustomFilter::new(move |obj| {
        let filter_state = filter_state_clone.borrow(); if let Ok(file_item) = <Object as Clone>::clone(obj).downcast::<FileDataItem>() { let original_name = file_item.property::<String>("original-name"); if !filter_state.excluded_extensions.is_empty() { if let Some(dot_pos) = original_name.rfind('.') { let extension = &original_name[dot_pos + 1..].to_lowercase(); if filter_state.excluded_extensions.contains(extension) { return false; } } } if filter_state.filename_regex_enabled { if let Some(regex) = &filter_state.filename_regex { if !regex.is_match(&original_name) { return false; } } } true } else { true }
    });
    filter_model.set_filter(Some(&custom_filter));

    let update_filter = {
        let filter_state = filter_state.clone();
        let excluded_extensions_entry = excluded_extensions_entry.clone();
        let filename_regex_check = filename_regex_check.clone();
        let filename_regex_entry = filename_regex_entry.clone();
        let custom_filter_clone = custom_filter.clone();
        move || {
            // Scope interne pour borrow_mut()
            {
                let mut state = filter_state.borrow_mut();
                state.excluded_extensions = excluded_extensions_entry.text().split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect();
                state.filename_regex_enabled = filename_regex_check.is_active();
                state.filename_regex_str = filename_regex_entry.text().to_string();
                state.filename_regex = if state.filename_regex_enabled && !state.filename_regex_str.is_empty() { match Regex::new(&state.filename_regex_str) { Ok(re) => Some(re), Err(e) => { eprintln!("Erreur regex: {}", e); None } } } else { None };
            } // Fin du scope pour borrow_mut()
            custom_filter_clone.changed(FilterChange::Different); // Appel sur le filtre
            println!("Refiltrage demandé.");
        }
    };

    // --- Connexions de signaux pour filtres (Complètes) ---
    excluded_extensions_entry.connect_changed(clone!(@strong update_filter => move |_| { update_filter(); }));
    filename_regex_entry.connect_changed(clone!(@strong update_filter => move |_| { update_filter(); }));
    filename_regex_check.connect_toggled(move |_| { update_filter(); });


    println!("DEBUG: build_ui() - Avant window.present()");
    window.present();
    println!("DEBUG: build_ui() - Après window.present()");
} // Fin de build_ui