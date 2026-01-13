use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use std::fs;
use zip::{ZipWriter, ZipArchive};
use walkdir::WalkDir;
use std::io::Write;

#[derive(Default)]
struct ZipGuiApp {
    selected_files: Vec<PathBuf>,
    selected_zip: Option<PathBuf>,
    status_message: String,
    operation_mode: OperationMode,
}

#[derive(Default, PartialEq)]
enum OperationMode {
    #[default]
    Zip,
    Unzip,
}

impl eframe::App for ZipGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zip/Unzip Tool");
            
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.operation_mode, OperationMode::Zip, "Zip Files");
                ui.selectable_value(&mut self.operation_mode, OperationMode::Unzip, "Unzip Files");
            });

            ui.separator();

            match self.operation_mode {
                OperationMode::Zip => {
                    ui.heading("Zip Mode");
                    
                    if ui.button("Select Files to Zip").clicked() {
                        if let Some(files) = FileDialog::new()
                            .add_filter("All files", &["*"])
                            .pick_files() {
                            let file_count = files.len();
                            self.selected_files = files;
                            self.status_message = format!("Selected {} files", file_count);
                        }
                    }

                    if !self.selected_files.is_empty() {
                        ui.label(format!("Selected {} files:", self.selected_files.len()));
                        for file in &self.selected_files {
                            ui.label(format!("  {}", file.display()));
                        }
                    }

                    if ui.button("Zip Selected Files").clicked() && !self.selected_files.is_empty() {
                        if let Some(save_path) = FileDialog::new()
                            .add_filter("Zip files", &["zip"])
                            .save_file() {
                            match zip_files(&self.selected_files, &save_path) {
                                Ok(_) => {
                                    self.status_message = format!("Successfully zipped to {}", save_path.display());
                                    self.selected_files.clear();
                                }
                                Err(e) => {
                                    self.status_message = format!("Error zipping: {}", e);
                                }
                            }
                        }
                    }
                }
                OperationMode::Unzip => {
                    ui.heading("Unzip Mode");
                    
                    if ui.button("Select Zip File").clicked() {
                        if let Some(file) = FileDialog::new()
                            .add_filter("Zip files", &["zip"])
                            .pick_file() {
                            self.selected_zip = Some(file.clone());
                            self.status_message = format!("Selected zip file: {}", file.display());
                        }
                    }

                    if let Some(zip_path) = &self.selected_zip {
                        ui.label(format!("Selected: {}", zip_path.display()));
                        
                        if ui.button("Choose Extract Location").clicked() {
                            if let Some(extract_dir) = FileDialog::new()
                                .pick_folder() {
                                match unzip_file(zip_path, &extract_dir) {
                                    Ok(_) => {
                                        self.status_message = format!("Successfully extracted to {}", extract_dir.display());
                                    }
                                    Err(e) => {
                                        self.status_message = format!("Error extracting: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ui.separator();
            ui.label(&self.status_message);
        });
    }
}

fn zip_files(files: &[PathBuf], output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::create(output_path)?;
    let mut zip = ZipWriter::new(file);
    
    for file_path in files {
        if file_path.is_file() {
            let mut file = fs::File::open(file_path)?;
            let contents = std::fs::read(file_path)?;
            
            let zip_path = file_path.file_name()
                .ok_or("Invalid filename")?;
            
            zip.start_file(zip_path.to_string_lossy(), zip::write::FileOptions::default())?;
            zip.write_all(&contents)?;
        } else if file_path.is_dir() {
            for entry in WalkDir::new(file_path) {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    let contents = std::fs::read(path)?;
                    let relative_path = path.strip_prefix(file_path)?;
                    let zip_path = relative_path.to_string_lossy();
                    
                    zip.start_file(zip_path.as_ref(), zip::write::FileOptions::default())?;
                    zip.write_all(&contents)?;
                }
            }
        }
    }
    
    zip.finish()?;
    Ok(())
}

fn unzip_file(zip_path: &PathBuf, extract_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = extract_dir.join(file.mangled_name());

        if (file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    
    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_title("Zip/Unzip GUI"),
        ..Default::default()
    };

    eframe::run_native(
        "Zip GUI",
        options,
        Box::new(|_cc| Box::<ZipGuiApp>::default()),
    )
}