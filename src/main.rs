mod logic;

use eframe::egui;
use logic::{ImageFile, collect_image_files, parse_date_flexible};
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
    show_debug_column: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                    thread::spawn(move || {
                        for img in &mut files {
                            if cancel_flag.load(Ordering::SeqCst) {
                                break;
                            }

                            if let Some(new_name) = &img.preview_name {
                                let new_path = img.path.parent().unwrap_or(Path::new(".")).join(new_name);
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
                ui.checkbox(&mut self.show_debug_column, "🛠️ Afficher debug format?");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(path) = &self.folder_path {
                ui.label(format!("📂 Dossier sélectionné : {}", path.display()));
            }

            if !self.image_files.is_empty() {
                ui.separator();

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
                        ui.label("Date EXIF = Nom");
                        ui.label("Nouveau nom");
                        if self.show_debug_column {
                            ui.label("🛠️ Formats testés");
                        }
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

                            if img.exif_date_matches_name {
                                ui.label("✅ identique");
                            } else {
                                ui.label("❌ différente");
                            }

                            if let Some(preview) = &img.preview_name {
                                ui.label(preview);
                            } else {
                                ui.label("-");
                            }

                            if self.show_debug_column {
                                if let Some(date) = &img.date_taken {
                                    if let Some(parsed) = parse_date_flexible(date) {
                                        let all_formats = vec![
                                            parsed.format("%Y-%m-%d_%H-%M-%S").to_string(),
                                            parsed.format("%Y%m%d_%H%M%S").to_string(),
                                            parsed.format("%Y_%m_%d_%H%M%S").to_string(),
                                            parsed.format("%Y.%m.%d_%H-%M-%S").to_string()
                                        ];
                                        ui.label(all_formats.join("\n"));
                                    } else {
                                        ui.label("(err parse)");
                                    }
                                } else {
                                    ui.label("(no date)");
                                }
                            }

                            ui.end_row();
                        }
                    });
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("TimeShotRenamer", options, Box::new(|_cc| Box::new(MyApp::default())))
}
