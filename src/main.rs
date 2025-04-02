use eframe::egui;
use rfd::FileDialog;
use walkdir::WalkDir;
use std::fs::{self, File};
use std::path::PathBuf;
use exif::{Reader, Tag};

struct ImageFile {
    path: PathBuf,
    date_taken: Option<String>,
    date_in_name: bool,
}

struct MyApp {
    folder_path: Option<PathBuf>,
    image_files: Vec<ImageFile>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            folder_path: None,
            image_files: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Renommage de photos basé sur l'EXIF");
            if ui.button("📁 Choisir un dossier").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    println!("📂 Dossier sélectionné : {}", path.display());
                    self.folder_path = Some(path.clone());
                    self.image_files = collect_image_files(&path);
                } else {
                    println!("❌ Aucun dossier sélectionné.");
                }
            }

            if let Some(folder) = &self.folder_path {
                ui.label(format!("Dossier sélectionné : {}", folder.display()));
                if ui.button("🔄 Renommer les fichiers").clicked() {
                    for img in &self.image_files {
                        if let Some(date) = &img.date_taken {
                            let formatted = date.replace(":", "-").replace(" ", "_");
                            let original_name = img.path.file_name().unwrap().to_string_lossy();
                            let new_name = format!("{}_{}", formatted, original_name);
                            let new_path = img.path.parent().unwrap().join(new_name);
                            println!("Renommage : {} -> {}", img.path.display(), new_path.display());
                            if let Err(e) = fs::rename(&img.path, &new_path) {
                                eprintln!("Erreur de renommage : {}", e);
                            }
                        }
                    }
                    // Recharger les fichiers après renommage
                    if let Some(path) = &self.folder_path {
                        self.image_files = collect_image_files(path);
                    }
                }
            }

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.image_files.is_empty() {
                    ui.label("Aucun fichier trouvé.");
                } else {
                    for img in &self.image_files {
                        ui.group(|ui| {
                            ui.label(format!("📄 Nom du fichier : {}", img.path.file_name().unwrap().to_string_lossy()));
                            if let Some(date) = &img.date_taken {
                                ui.label(format!("📸 Date EXIF : {}", date));
                            } else {
                                ui.label("⚠️ Pas de date EXIF trouvée");
                            }
                            if img.date_in_name {
                                ui.label("✅ La date est présente dans le nom du fichier");
                            } else {
                                ui.label("❌ La date ne semble pas présente dans le nom du fichier");
                            }
                        });
                        ui.separator();
                    }
                }
            });
        });
    }
}

fn collect_image_files(dir: &PathBuf) -> Vec<ImageFile> {
    println!("📥 Scan du dossier : {}", dir.display());

    let mut files = vec![];

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            println!("→ Fichier trouvé : {}", path.display());
            let date_taken = read_exif_date(path);
            let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
            let date_in_name = match &date_taken {
                Some(date) => is_date_in_filename(&file_name, date),
                None => false,
            };

            files.push(ImageFile {
                path: path.to_path_buf(),
                date_taken,
                date_in_name,
            });
        }
    }

    println!("✔️ Total fichiers trouvés : {}", files.len());
    files
}

fn read_exif_date(path: &std::path::Path) -> Option<String> {
    if let Ok(file) = File::open(path) {
        if let Ok(reader) = Reader::new().read_from_container(&mut std::io::BufReader::new(file)) {
            if let Some(field) = reader.get_field(Tag::DateTimeOriginal, exif::In::PRIMARY) {
                return Some(field.display_value().with_unit(&reader).to_string());
            }
        }
    }
    None
}

fn is_date_in_filename(file_name: &str, exif_date: &str) -> bool {
    let date_clean = exif_date
        .replace(":", "")
        .replace(" ", "")
        .replace("-", "")
        .replace("h", "")
        .replace("m", "")
        .replace("s", "")
        .to_lowercase();

    let patterns = vec![
        date_clean.clone(),
        date_clean.chars().take(8).collect::<String>(),
        date_clean.chars().skip(8).collect::<String>(),
        date_clean.chars().skip(2).take(6).collect::<String>(),
        date_clean.chars().take(4).collect::<String>(),
    ];

    for p in patterns {
        if file_name.contains(&p) {
            return true;
        }
    }

    false
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Photo Renamer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
