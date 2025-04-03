// src/main.rs
mod logic;

use eframe::egui;
use logic::{ImageFile, collect_image_files, generate_preview_name};
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Default)]
struct MyApp {
    folder_path: Option<PathBuf>,
    image_files: Vec<ImageFile>,
    select_all: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📁 Choisir un dossier").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.folder_path = Some(path.clone());
                        self.image_files = collect_image_files(&path);
                    }
                }

                if ui.button("✅ Sélectionner les fichiers avec EXIF").clicked() {
                    for img in &mut self.image_files {
                        img.selected = img.date_taken.is_some();
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(path) = &self.folder_path {
                ui.label(format!("📂 Dossier sélectionné : {}", path.display()));
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("files_grid").striped(true).show(ui, |ui| {
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

                        if !img.preview_valid {
                            img.preview_name = Some(generate_preview_name(&img.file_name, &img.date_taken));
                            img.preview_valid = true;
                        }

                        if let Some(preview) = &img.preview_name {
                            ui.label(preview);
                        } else {
                            ui.label("-");
                        }

                        ui.end_row();
                    }
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("TimeShotRenamer", options, Box::new(|_cc| Box::new(MyApp::default())))
}
