#![windows_subsystem = "windows"]

mod logic;

use eframe::egui;
use logic::{ImageFile, collect_image_files, generate_preview_name};
use rfd::FileDialog;
use std::fs;
use std::path::{PathBuf, Path};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct MyApp {
    folder_path: Option<PathBuf>,
    image_files: Vec<ImageFile>,
    select_all: bool,
    dry_run: bool,
    status_messages: Vec<String>,
    is_processing: bool,
    progress: Option<(usize, usize)>,
    cancel_flag: Arc<AtomicBool>,
    selected_exif_field: Option<String>,
    exif_filter: String,
    insertion_offset: usize,
    include_subfolders: bool,
    show_exif_popup: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let window_size = ctx.screen_rect();

        if let Some(drop) = ctx.input(|i| i.raw.dropped_files.first().cloned()) {
            if let Some(path) = drop.path {
                if path.is_dir() {
                    self.folder_path = Some(path.clone());
                    self.image_files = collect_image_files(&path, self.include_subfolders);
                    self.status_messages.clear();
                }
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("📁 Choisir un dossier").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.folder_path = Some(path.clone());
                        self.image_files = collect_image_files(&path, self.include_subfolders);
                        self.status_messages.clear();
                    }
                }

                ui.checkbox(&mut self.include_subfolders, "Inclure les sous-dossiers");

                if ui.button("✅ Sélectionner les fichiers avec EXIF").clicked() {
                    for img in &mut self.image_files {
                        img.selected = img.date_taken.is_some();
                    }
                }

                if ui.button("🔄 Renommer les fichiers sélectionnés").clicked() && !self.is_processing {
                    self.status_messages.clear();
                    self.is_processing = true;
                    self.cancel_flag.store(false, Ordering::SeqCst);
                    let dry_run = self.dry_run;
                    let cancel_flag = self.cancel_flag.clone();
                    let mut files: Vec<_> = self.image_files.clone().into_iter().filter(|f| f.selected).collect();
                    let ctx = ctx.clone();
                    let selected_field = self.selected_exif_field.clone();
                    let offset = self.insertion_offset;
                    let total = files.len();

                    thread::spawn(move || {
                        for (idx, img) in files.iter_mut().enumerate() {
                            if cancel_flag.load(Ordering::SeqCst) {
                                break;
                            }

                            if let Some(mut new_name) = img.preview_name.clone() {
                                if let Some(ref field) = selected_field {
                                    if let Some(value) = img.exif_data.get(field) {
                                        let (prefix, suffix) = new_name.split_at(offset.min(new_name.len()));
                                        new_name = format!("{}{}_{}", prefix, value.replace(' ', "_"), suffix);
                                    }
                                }

                                let mut new_path = img.path.parent().unwrap_or(Path::new(".")).join(&new_name);
                                let mut counter = 1;
                                while new_path.exists() {
                                    let stem = new_path.file_stem().unwrap_or_default().to_string_lossy();
                                    let ext = new_path.extension().unwrap_or_default().to_string_lossy();
                                    let new_file_name = format!("{}_{}.{}", stem, counter, ext);
                                    new_path = new_path.parent().unwrap_or(Path::new(".")).join(new_file_name);
                                    counter += 1;
                                }

                                if !dry_run {
                                    let _ = fs::rename(&img.path, &new_path);
                                }
                            }

                            ctx.request_repaint();
                        }
                    });
                }

                if self.is_processing {
                    if ui.button("⛔ Annuler").clicked() {
                        self.cancel_flag.store(true, Ordering::SeqCst);
                    }
                }

                ui.checkbox(&mut self.dry_run, "Simulation uniquement (dry run)");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(path) = &self.folder_path {
                ui.label(format!("📂 Dossier sélectionné : {}", path.display()));
            }

            if !self.image_files.is_empty() {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("🔍 Filtrer un champ EXIF :");
                    ui.text_edit_singleline(&mut self.exif_filter);
                    ui.add(egui::Slider::new(&mut self.insertion_offset, 0..=50).text("Insérer après X caractères"));
                    if ui.button("📋 Choisir un champ EXIF").clicked() {
                        self.show_exif_popup = true;
                    }
                });

                if self.show_exif_popup {
                    egui::Window::new("Sélection du champ EXIF")
                        .collapsible(false)
                        .resizable(true)
                        .default_size([400.0, 300.0])
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("🔎 Rechercher :");
                                ui.text_edit_singleline(&mut self.exif_filter);
                            });

                            let all_tags: HashSet<String> = self.image_files.iter()
                                .flat_map(|f| f.exif_data.keys().cloned())
                                .collect();

                            let mut grouped: HashMap<&str, Vec<String>> = HashMap::new();
                            for tag in all_tags {
                                let group = if tag.to_lowercase().contains("date") {
                                    "📅 Dates"
                                } else if ["make", "model", "lens", "iso", "focallength"].iter().any(|k| tag.to_lowercase().contains(k)) {
                                    "📸 Appareil"
                                } else if tag.to_lowercase().contains("desc") || tag.to_lowercase().contains("title") || tag.to_lowercase().contains("keyword") {
                                    "📝 Descriptions"
                                } else {
                                    "📁 Autres"
                                };
                                if self.exif_filter.is_empty() || tag.to_lowercase().contains(&self.exif_filter.to_lowercase()) {
                                    grouped.entry(group).or_default().push(tag);
                                }
                            }

                            egui::ScrollArea::vertical().max_height(window_size.height() * 0.5).show(ui, |ui| {
                                ui.selectable_value(&mut self.selected_exif_field, None, "(aucun)".to_string());
                                for (category, tags) in &grouped {
                                    ui.separator();
                                    ui.label(egui::RichText::new(*category).strong());
                                    for tag in tags {
                                        if ui.selectable_label(self.selected_exif_field == Some(tag.clone()), tag).clicked() {
                                            self.selected_exif_field = Some(tag.clone());
                                            self.show_exif_popup = false;
                                        }
                                    }
                                }
                            });
                        });
                }
            }

            egui::ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                egui::Grid::new("files_grid").striped(true).min_col_width(150.0).show(ui, |ui| {
                    if ui.checkbox(&mut self.select_all, "").changed() {
                        for img in &mut self.image_files {
                            img.selected = self.select_all;
                        }
                    }
                    ui.label("Nom");
                    ui.label("EXIF");
                    ui.label("Date dans nom");
                    ui.label("Nouveau nom");
                    ui.end_row();

                    for img in &mut self.image_files {
                        ui.checkbox(&mut img.selected, "");
                        ui.label(&img.file_name);

                        if let Some(date) = &img.date_taken {
                            ui.label(format!("✅ {}", date));
                        } else {
                            ui.colored_label(egui::Color32::RED, "❌");
                        }

                        if img.date_in_name {
                            ui.label("✅ présente");
                        } else {
                            ui.label("❌ absente");
                        }

                        let dynamic_preview = if let Some(base_name) = &img.preview_name {
                            if let Some(field) = &self.selected_exif_field {
                                if let Some(value) = img.exif_data.get(field) {
                                    let (prefix, suffix) = base_name.split_at(self.insertion_offset.min(base_name.len()));
                                    Some(format!("{}{}_{}", prefix, value.replace(' ', "_"), suffix))
                                } else {
                                    Some(base_name.clone())
                                }
                            } else {
                                Some(base_name.clone())
                            }
                        } else {
                            None
                        };

                        if let Some(preview) = dynamic_preview {
                            ui.label(preview);
                        } else {
                            ui.label("-");
                        }

                        ui.end_row();
                    }
                });
            });

            if let Some((done, total)) = self.progress {
                let progress = done as f32 / total as f32;
                ui.add(egui::ProgressBar::new(progress).text(format!("Progression : {}/{}", done, total)));
            }

            if !self.status_messages.is_empty() {
                ui.separator();
                ui.label("📋 Résultat du renommage :");
                for msg in &self.status_messages {
                    ui.label(msg);
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("TimeShotRenamer", options, Box::new(|_cc| Box::new(MyApp::default())))
}
