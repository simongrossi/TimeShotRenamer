// timeshot_gui/src/ui.rs

use crate::file_data_item::FileDataItem;
use gtk4::gio::ListStore;
use gtk4::glib::{self, clone};
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, CssProvider, DialogFlags,
    FileChooserAction, FileChooserDialog, Label, ListItem, ListView, MessageDialog, MessageType,
    NoSelection, Orientation, ResponseType, ScrolledWindow, SignalListItemFactory, ButtonsType,
};
use gtk4::pango;
use std::{fs, path::PathBuf}; // Ajout pour fs::rename et PathBuf

use timeshot_core::analyze_and_prepare_files;

pub fn build_ui(app: &Application) {
    println!("DEBUG: build_ui() - Début");

    // --- CSS ---
    let provider = CssProvider::new();
    provider.load_from_data("label.duplicate { color: orange; font-style: italic; } label.heading { font-weight: bold; } button.destructive-action { background-color: #e74c3c; color: white; }"); // Ajout CSS pour bouton
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // --- Fenêtre principale ---
    let window = ApplicationWindow::builder()
        .application(app)
        .title("TimeShotRenamer GTK")
        .default_width(1000)
        .default_height(700)
        .build();

    // --- Widgets principaux (RESTAURÉS) ---
    let main_vbox = GtkBox::new(Orientation::Vertical, 10);
    main_vbox.set_margin_top(10);
    main_vbox.set_margin_bottom(10);
    main_vbox.set_margin_start(10);
    main_vbox.set_margin_end(10);

    let top_hbox = GtkBox::new(Orientation::Horizontal, 6);
    let open_button = Button::with_label("Ouvrir un dossier");
    let path_label = Label::new(Some("Aucun dossier sélectionné"));
    path_label.set_halign(Align::Start);
    path_label.set_hexpand(true);
    top_hbox.append(&open_button);
    top_hbox.append(&path_label);

    let options_hbox = GtkBox::new(Orientation::Horizontal, 6);
    options_hbox.set_margin_top(5);
    let recursive_checkbox = CheckButton::with_label("Inclure les sous-dossiers");
    recursive_checkbox.set_active(false);
    options_hbox.append(&recursive_checkbox);

    let model = ListStore::new::<FileDataItem>();

    let factory = SignalListItemFactory::new();
    // --- Factory setup (RESTAURÉ) ---
    factory.connect_setup(move |_factory, list_item| {
        let item_widget = GtkBox::new(Orientation::Horizontal, 10);
        item_widget.set_margin_top(3);
        item_widget.set_margin_bottom(3);
        let check = CheckButton::new();
        let original_name_label = Label::builder().halign(Align::Start).hexpand(true).wrap(true).wrap_mode(pango::WrapMode::WordChar).build();
        let proposed_name_label = Label::builder().halign(Align::Start).hexpand(true).wrap(true).wrap_mode(pango::WrapMode::WordChar).build();
        let date_label = Label::builder().halign(Align::Start).width_chars(19).build();
        let duplicate_label = Label::builder().halign(Align::End).width_chars(10).build();
        item_widget.append(&check);
        item_widget.append(&original_name_label);
        item_widget.append(&proposed_name_label);
        item_widget.append(&date_label);
        item_widget.append(&duplicate_label);
        list_item.downcast_ref::<ListItem>().expect("ListItem requis").set_child(Some(&item_widget));
    });
    // --- Factory bind (RESTAURÉ + avec connect_notify_local) ---
    factory.connect_bind(move |_factory, list_item| {
        let list_item_gtk = list_item.downcast_ref::<ListItem>().expect("Needs ListItem");
        let file_item = list_item_gtk.item().and_downcast::<FileDataItem>().expect("Needs FileDataItem");
        let item_widget = list_item_gtk.child().and_downcast::<GtkBox>().expect("Needs GtkBox");
        let check = item_widget.first_child().and_downcast::<CheckButton>().expect("Needs CheckButton");
        let original_name_label = check.next_sibling().and_downcast::<Label>().expect("Needs Label Original Name");
        let proposed_name_label = original_name_label.next_sibling().and_downcast::<Label>().expect("Needs Label Proposed Name");
        let date_label = proposed_name_label.next_sibling().and_downcast::<Label>().expect("Needs Label Date");
        let duplicate_label = date_label.next_sibling().and_downcast::<Label>().expect("Needs Label Duplicate");

        original_name_label.set_text(&file_item.property::<String>("original-name"));
        let proposed_str = file_item.property::<String>("proposed-name");
        proposed_name_label.set_text(if proposed_str.is_empty() { "-" } else { &proposed_str });
        let date_str = file_item.property::<String>("date-taken");
        date_label.set_text(if date_str.is_empty() { "-" } else { &date_str });

        if file_item.property::<bool>("is-duplicate") {
            duplicate_label.set_text("Doublon");
            duplicate_label.add_css_class("duplicate");
            original_name_label.add_css_class("duplicate");
        } else {
            duplicate_label.set_text("");
            duplicate_label.remove_css_class("duplicate");
            original_name_label.remove_css_class("duplicate");
        }

        check.set_active(file_item.property::<bool>("selected"));

        file_item.connect_notify_local(Some("selected"), clone!(@weak check => move |item, _pspec| {
            let is_selected = item.property::<bool>("selected");
             println!("NOTIFY::selected: Item '{}' changed to {}", item.property::<String>("original-name"), is_selected);
             check.set_active(is_selected);
        }));

        check.connect_toggled(clone!(@weak file_item => move |check_button| {
            file_item.set_property("selected", check_button.is_active());
        }));
    });

    // --- ListView et ScrolledWindow (RESTAURÉS) ---
    let selection_model = NoSelection::new(Some(model.clone()));
    let list_view = ListView::new(Some(selection_model), Some(factory));
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_height(400)
        .child(&list_view)
        .vexpand(true)
        .build();

    // --- Header et Boutons de sélection (RESTAURÉS) ---
    let header_box = GtkBox::new(Orientation::Horizontal, 10); /* ... Définition ... */
    header_box.set_margin_start(35);
    let header_orig = Label::builder().label("Nom Original").halign(Align::Start).hexpand(true).css_classes(vec!["heading".to_string()]).build();
    let header_prop = Label::builder().label("Nom Proposé").halign(Align::Start).hexpand(true).css_classes(vec!["heading".to_string()]).build();
    let header_date = Label::builder().label("Date Prise").halign(Align::Start).width_chars(19).css_classes(vec!["heading".to_string()]).build();
    let header_dup = Label::builder().label("Statut").halign(Align::End).width_chars(10).css_classes(vec!["heading".to_string()]).build();
    header_box.append(&header_orig); header_box.append(&header_prop); header_box.append(&header_date); header_box.append(&header_dup);

    let selection_hbox = GtkBox::new(Orientation::Horizontal, 6); /* ... Définition ... */
    selection_hbox.set_halign(Align::Start);
    let select_all_button = Button::with_label("Tout Sélectionner");
    let deselect_all_button = Button::with_label("Tout Désélectionner");
    let select_exif_button = Button::with_label("Sélectionner si Date EXIF");
    selection_hbox.append(&select_all_button); selection_hbox.append(&deselect_all_button); selection_hbox.append(&select_exif_button);


    // --- Barre de boutons d'action (Renommer) (RESTAURÉE ET AJOUTÉE) ---
    let action_hbox = GtkBox::new(Orientation::Horizontal, 6);
    action_hbox.set_halign(Align::End);
    action_hbox.set_margin_top(10);
    let rename_button = Button::with_label("Renommer Sélection");
    rename_button.add_css_class("destructive-action");
    action_hbox.append(&rename_button);

    // --- Assemblage UI (RESTAURÉ ET COMPLET) ---
    main_vbox.append(&top_hbox);
    main_vbox.append(&options_hbox);
    main_vbox.append(&header_box);
    main_vbox.append(&selection_hbox);
    main_vbox.append(&scrolled_window);
    main_vbox.append(&action_hbox); // Ajout de la barre d'action en bas

    window.set_child(Some(&main_vbox)); // window est maintenant défini

    // --- Logique bouton "Ouvrir dossier" (RESTAURÉE et fonctionnelle) ---
    let window_clone_open = window.clone(); // Cloner la 'window' définie plus haut
    let model_button_clone = model.clone();
    let path_label_clone = path_label.clone();
    let recursive_checkbox_clone = recursive_checkbox.clone();
    open_button.connect_clicked(move |_| {
         // !! Arguments Corrigés !!
         let dialog = FileChooserDialog::new(
            Some("Choisir un dossier"),
            Some(&window_clone_open), // Utilise le bon clone
            FileChooserAction::SelectFolder, // FileChooserAction utilisé
            &[("Annuler", ResponseType::Cancel), ("Ouvrir", ResponseType::Accept)],
        );
        let model_dialog_clone = model_button_clone.clone();
        let path_label_dialog_clone = path_label_clone.clone();
        let recursive_checkbox_dialog_clone = recursive_checkbox_clone.clone();
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept { if let Some(file) = dialog.file() { if let Some(path) = file.path() {
                        path_label_dialog_clone.set_text(&format!("Dossier : {}", path.display()));
                        model_dialog_clone.remove_all();
                        let recursive_scan = recursive_checkbox_dialog_clone.is_active();
                        println!("Appel de analyze_and_prepare_files pour : {}, Récursif: {}", path.display(), recursive_scan);
                        match analyze_and_prepare_files(&path, recursive_scan) {
                            Ok(analysis_results) => {
                                println!("Analyse réussie, {} résultats.", analysis_results.len());
                                if analysis_results.is_empty() { println!("Aucun fichier trouvé ou analysable dans le dossier."); }
                                for result in analysis_results { let item = FileDataItem::from_analysis(&result); model_dialog_clone.append(&item); }
                            }
                            Err(e) => { eprintln!("Erreur lors de l'analyse du dossier : {}", e); path_label_dialog_clone.set_text(&format!("Erreur analyse: {}", e)); }
                        }
                    } else { path_label_dialog_clone.set_text("Erreur : Chemin invalide sélectionné"); model_dialog_clone.remove_all(); }
                } else { path_label_dialog_clone.set_text("Erreur : Aucun dossier sélectionné"); model_dialog_clone.remove_all(); }
            }
            dialog.close();
        });
        dialog.show();
    });

    // --- Logique boutons sélection (RESTAURÉE et fonctionnelle) ---
    let model_select_all = model.clone();
    select_all_button.connect_clicked(move |_| { /* ... identique (avec set_property, sans items_changed) ... */
        println!("Bouton 'Tout Sélectionner' cliqué");
        for i in 0..model_select_all.n_items() {
            if let Some(obj) = model_select_all.item(i) {
                 if let Ok(item) = obj.downcast::<FileDataItem>() { item.set_property("selected", true); }
            }
        }
        println!("Fin 'Tout Sélectionner'");
    });
    let model_deselect_all = model.clone();
    deselect_all_button.connect_clicked(move |_| { /* ... identique ... */
        println!("Bouton 'Tout Désélectionner' cliqué");
        for i in 0..model_deselect_all.n_items() {
            if let Some(obj) = model_deselect_all.item(i) {
                 if let Ok(item) = obj.downcast::<FileDataItem>() { item.set_property("selected", false); }
            }
        }
        println!("Fin 'Tout Désélectionner'");
    });
    let model_select_exif = model.clone();
    select_exif_button.connect_clicked(move |_| { /* ... identique ... */
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

    // --- Logique bouton Renommer (RESTAURÉE et AJOUTÉE) ---
    let model_rename = model.clone();
    let window_clone_rename = window.clone(); // window est maintenant défini
    rename_button.connect_clicked(move |_| {
        println!("Bouton 'Renommer Sélection' cliqué");
        let mut items_to_rename: Vec<(PathBuf, PathBuf, u32)> = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        let mut success_count = 0;
        let mut skipped_count = 0;

        for i in 0..model_rename.n_items() {
            if let Some(obj) = model_rename.item(i) {
                if let Ok(item) = obj.downcast::<FileDataItem>() {
                    if item.property::<bool>("selected") {
                        let original_path = item.full_original_path();
                        let proposed_name = item.property::<String>("proposed-name"); // Récupère Option<String> via GObject

                        if !proposed_name.is_empty() && proposed_name != "-" {
                             if let Some(parent_dir) = original_path.parent() {
                                let new_path = parent_dir.join(&proposed_name);
                                if original_path != new_path {
                                    items_to_rename.push((original_path.clone(), new_path, i)); // Cloner original_path si nécessaire
                                } else { skipped_count += 1; /* log skip */ }
                            } else { errors.push(format!("Pas de parent pour: {}", original_path.display())); }
                        } else { skipped_count += 1; /* log skip */ }
                    }
                }
            }
        }

        if items_to_rename.is_empty() && errors.is_empty() {
             println!("Aucun fichier sélectionné ou valide pour le renommage.");
             let dialog = MessageDialog::new( Some(&window_clone_rename), DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT, MessageType::Info, ButtonsType::Ok, "Aucun fichier valide sélectionné pour le renommage.");
             dialog.connect_response(|d, _| d.close());
             dialog.show();
             return;
        }

        println!("Tentative de renommage de {} fichier(s)...", items_to_rename.len());
        let mut indices_to_remove = Vec::new();

        // Itérer en ordre inverse sur les index pour éviter les problèmes de décalage lors de la suppression
         for item_info in items_to_rename.iter().rev() {
            let original_path = &item_info.0;
            let new_path = &item_info.1;
            let model_index = item_info.2;
            println!("Renommage de {} -> {}", original_path.display(), new_path.display());
            match fs::rename(original_path, new_path) {
                Ok(_) => { success_count += 1; indices_to_remove.push(model_index); }
                Err(e) => { let error_msg = format!("Erreur renommage '{}': {}", original_path.display(), e); eprintln!("{}", error_msg); errors.push(error_msg); }
            }
        }

        // Suppression des éléments renommés du modèle
        for index in &indices_to_remove { model_rename.remove(*index); }

        // Affichage résumé
        let mut summary = format!("Renommage terminé.\n\nSuccès : {}\nÉchecs : {}\nSkippés : {}\n", success_count, errors.len(), skipped_count);
        if !errors.is_empty() { /* ... ajout détails erreurs ... */
             summary.push_str("\nDétails des erreurs :\n");
            for err in errors.iter().take(10) { summary.push_str(&format!("- {}\n", err)); }
            if errors.len() > 10 { summary.push_str("...\n"); }
        }
        let dialog = MessageDialog::new( Some(&window_clone_rename), DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT, if errors.is_empty() { MessageType::Info } else { MessageType::Warning }, ButtonsType::Ok, &summary);
        dialog.connect_response(|d, _| d.close());
        dialog.show();
        println!("Fin 'Renommer Sélection'");
    });

    println!("DEBUG: build_ui() - Avant window.present()");
    window.present(); // window est maintenant défini
    println!("DEBUG: build_ui() - Après window.present()");
}