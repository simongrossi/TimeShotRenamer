// timeshot_gui/src/file_data_item.rs

use gtk4::glib::{self, Object, ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};
use std::path::PathBuf; // <-- Ajout PathBuf
use timeshot_core::types::FileAnalysis;

// Module interne
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct FileDataItem {
        // !! NOUVEAU CHAMP INTERNE (non exposé comme propriété GObject) !!
        pub full_original_path: RefCell<PathBuf>,
        // Propriétés GObject existantes
        pub original_name: RefCell<String>,
        pub proposed_name: RefCell<Option<String>>,
        pub folder_name: RefCell<String>,
        pub date_taken: RefCell<Option<String>>,
        pub is_duplicate: Cell<bool>,
        pub selected: Cell<bool>,
        pub file_hash: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileDataItem {
        const NAME: &'static str = "TimeshotFileDataItem";
        type Type = super::FileDataItem;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for FileDataItem {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("original-name").build(),
                    ParamSpecString::builder("proposed-name").build(),
                    ParamSpecString::builder("folder-name").build(),
                    ParamSpecString::builder("date-taken").build(),
                    ParamSpecBoolean::builder("is-duplicate").build(),
                    ParamSpecBoolean::builder("selected").build(),
                    ParamSpecString::builder("file-hash").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
             match pspec.name() {
                "original-name" => self.original_name.borrow().to_value(),
                "proposed-name" => self.proposed_name.borrow().as_deref().unwrap_or("").to_value(),
                "folder-name" => self.folder_name.borrow().to_value(),
                "date-taken" => self.date_taken.borrow().as_deref().unwrap_or("").to_value(),
                "is-duplicate" => self.is_duplicate.get().to_value(),
                "selected" => self.selected.get().to_value(),
                "file-hash" => self.file_hash.borrow().as_deref().unwrap_or("").to_value(),
                _ => unimplemented!(),
            }
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
             match pspec.name() {
                "selected" => { if let Ok(sel) = value.get() { self.selected.set(sel); } }
                // Les autres ne sont normalement pas settés depuis l'UI pour l'instant
                 "original-name" => { if let Ok(name) = value.get() { *self.original_name.borrow_mut() = name;}}
                 "proposed-name" => { if let Ok(name) = value.get() { *self.proposed_name.borrow_mut() = name;}}
                 "folder-name" => { if let Ok(name) = value.get() { *self.folder_name.borrow_mut() = name;}}
                 "date-taken" => { if let Ok(date) = value.get() { *self.date_taken.borrow_mut() = date;}}
                 "is-duplicate" => { if let Ok(is_dup) = value.get() { self.is_duplicate.set(is_dup);}}
                 "file-hash" => { if let Ok(hash) = value.get() { *self.file_hash.borrow_mut() = hash;}}
                _ => unimplemented!(),
            }
        }
    }
} // Fin mod imp

// Wrapper public
glib::wrapper! {
    pub struct FileDataItem(ObjectSubclass<imp::FileDataItem>);
}

// Implémentation publique
impl FileDataItem {
    // Création à partir de FileAnalysis
    pub fn from_analysis(analysis: &FileAnalysis) -> Self {
        let date_str = analysis.exif.date_taken.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());

        // Crée l'objet GObject
        let obj: Self = Object::builder().build();

        // Remplit les champs internes (y compris le nouveau path)
        *obj.imp().full_original_path.borrow_mut() = analysis.full_original_path.clone();
        *obj.imp().original_name.borrow_mut() = analysis.original_name.clone();
        *obj.imp().proposed_name.borrow_mut() = analysis.new_name.clone();
        *obj.imp().folder_name.borrow_mut() = analysis.folder_name.clone();
        *obj.imp().date_taken.borrow_mut() = date_str;
        obj.imp().is_duplicate.set(analysis.is_duplicate);
        obj.imp().selected.set(false); // Non sélectionné par défaut
        *obj.imp().file_hash.borrow_mut() = analysis.file_hash.clone();

        obj // Retourne l'objet construit et rempli
    }

    // !! NOUVEAU : Getter public pour le chemin original complet !!
    pub fn full_original_path(&self) -> PathBuf {
        self.imp().full_original_path.borrow().clone()
    }
}