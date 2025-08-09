use eframe::{self, egui};
use crate::enc_dec;
use crate::file;
use std::thread::sleep;
use std::time::Duration;
use std::{thread};
use std::sync::{Mutex, Arc};



pub struct MainWindow {
    password: String,
    folder_or_file: String,
    
    keep_original_file: bool,
    in_process: bool,
    progress: Arc<Mutex<Option<f32>>>,
}


impl Default for MainWindow {
    fn default() -> Self {
        Self { 
            password: String::new(), 
            folder_or_file: String::new(), 

            keep_original_file: true,
            in_process: false,
            progress: Arc::new(Mutex::new(Some(0.0))),
        }
    }
}

fn encrpt<F>(password: String, from_path: String, mut report_progress: F)
where
    F: FnMut(f32),
{
    let encryptor = enc_dec::FileEncDecrpytor::new(password);
    let file_visitor = file::FileVisitor::new(from_path);

    let total = file_visitor.count_files();
    let mut count = 0;
    report_progress(0.0);

    file_visitor.visit_file(|p| {
        let origin_file = p.to_str().unwrap();
        let encrypted_file = format!("{}.enc", origin_file);
        if let Err(e) = encryptor.encrpt_file(origin_file, encrypted_file.as_str()) {
            println!("encrypt file {} failed : {}", p.display(), e.to_string());
        }
        count += 1;
        report_progress((count as f32)/(total as f32));
    });
}

fn decrpt<F>(password: String, from_path: String, mut report_progress: F)
where
    F: FnMut(f32),
{
    let encryptor = enc_dec::FileEncDecrpytor::new(password);
    let file_visitor = file::FileVisitor::new(from_path);

    let total = file_visitor.count_files();
    let mut count = 0;
    report_progress(0.0);

    file_visitor.visit_file(|p| {
        let origin_file = p.to_str().unwrap();
        if origin_file.ends_with(".enc"){
            let encrypted_file = &origin_file[..origin_file.len()-4];
            if let Err(e) = encryptor.decrpt_file(origin_file, encrypted_file) {
                println!("decrypt file {} failed : {}", p.display(), e.to_string());
            }
        }
        count += 1;
        report_progress((count as f32)/(total as f32));
    });
}


impl MainWindow {

    fn set_progress(&self, value: f32) {
        let progress = self.progress.clone();
        let mut res = progress.lock().unwrap();
        *res = Some(value);
    }

    fn start_task(&mut self) {
        self.in_process = true;
        self.set_progress(0.0);
    }

    fn on_encrypt(&self) {
        let password = self.password.clone();
        let from_path = self.folder_or_file.clone();

        let progress = self.progress.clone();
        thread::spawn(move || {
            encrpt(password, from_path, |val| {
                let mut res = progress.lock().unwrap();
                *res = Some(val);
            });
            sleep(Duration::new(1, 0));
            let mut res = progress.lock().unwrap();
            *res = None;
            println!("Encryption finished");
        });
    }
    
    fn on_decrypt(&mut self) {
        let password = self.password.clone();
        let from_path = self.folder_or_file.clone();

        let progress = self.progress.clone();
        thread::spawn(move || {
            decrpt(password, from_path, |val| {
                let mut res = progress.lock().unwrap();
                *res = Some(val);
            });
            sleep(Duration::new(1, 0));
            let mut res = progress.lock().unwrap();
            *res = None;
            println!("Decryption finished");
        });
    }
}


impl eframe::App for MainWindow {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx,  |ui| {
                egui::Frame::default()
                    .inner_margin(egui::Margin::symmetric(52, 32))
                    .show(ui, |ui| {
                        ui.heading("Secure Your Files");

                        // Password
                        ui.add_space(40.0);
                        ui.label(egui::RichText::new("Password:").size(14.0));

                        ui.add_space(8.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.password)
                                .desired_width(320.0)
                        );

                        // File Path
                        ui.add_space(16.0);
                        ui.label(egui::RichText::new("Folder path:").size(14.0));
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.folder_or_file)
                                    .interactive(false)
                                    .desired_width(320.0)
                            );
                            if ui.button("📂").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.folder_or_file = path.display().to_string();
                                }
                            }
                            if ui.button("📃").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    self.folder_or_file = path.display().to_string();
                                }
                            }
                        });

                        // Checkbox of removing original file option
                        ui.add_space(24.0);
                        ui.horizontal(|ui| {
                            ui.checkbox(
                                &mut self.keep_original_file, 
                                "Original files option"
                            );
                            if self.keep_original_file {
                                ui.label(
                                    egui::RichText::new("Original files will be kept")
                                        .color(egui::Color32::from_rgb(0, 120, 0))
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new("Original files will be removed")
                                        .color(egui::Color32::from_rgb(255, 0, 0))
                                );
                            }
                        });

                        // Encrpt / Decrpt Buttons
                        ui.add_space(40.0);
                        ui.horizontal(|ui| {
                            // encryption button
                            let encrypt_button = egui::Button::new("Encrypt Files")
                                .min_size(egui::Vec2::new(132.0, 24.0));
                            if ui.add(encrypt_button).clicked() {
                                self.start_task();
                                self.on_encrypt();
                            }
                            // decrpytion button
                            ui.add_space(32.0);
                            let decrpyt_button = egui::Button::new("Decrypt Files")
                                .min_size(egui::Vec2::new(132.0, 24.0));
                            if ui.add(decrpyt_button).clicked() {
                                self.start_task();
                                self.on_decrypt();
                            }
                        });
                    });
            });

        // Show popup window if encrypting or decrypting
        if self.in_process {
            let progress = *self.progress.lock().unwrap();
            egui::Window::new("🔄 Encrypting...")
                .collapsible(false)
                .resizable(false)
                .fixed_size(egui::vec2(300.0, 100.0))
                .show(ctx, |ui| {
                    if let Some(val) = progress {
                        ui.add(egui::ProgressBar::new(val).show_percentage());
                    } else {
                        self.in_process = false;
                    }
                    ctx.request_repaint();
                });
            }
    }
}
