use eframe::{self, egui};
use crate::enc_dec;
use crate::file::visit_all_files;


pub struct MainWindow {
    password: String,
    folder_path: String,

    in_process: bool,
    progress: f32,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self { 
            password: String::new(), 
            folder_path: String::new(), 
            in_process: false,
            progress: 0.0,
        }
    }
}

impl MainWindow {
    fn encrpt(&self) {
        println!("Encrypting files in folder: {}", self.folder_path);
        let encrptor = enc_dec::FileEncDecrpytor::new(self.password.clone());
        
        visit_all_files(&self.folder_path,|p| {
            let origin_file = p.to_str().unwrap();
            let encrypted_file = format!("{}.enc", origin_file);
            encrptor.encrpt_file(origin_file, encrypted_file.as_str());
        });
    }

    fn decrpt(&self) {
        println!("Decrypting files in folder: {}", self.folder_path);
        let encrptor = enc_dec::FileEncDecrpytor::new(self.password.clone());
        
        visit_all_files(&self.folder_path,|p| {
            let origin_file = p.to_str().unwrap();
            if origin_file.ends_with(".enc"){
                let encrypted_file = &origin_file[..origin_file.len()-4];
                encrptor.decrpt_file(origin_file, encrypted_file);
            }
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
                        ui.label(egui::RichText::new("File/Folder path:").size(14.0));
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.folder_path)
                                    .interactive(false)
                                    .desired_width(320.0)
                            );
                            if ui.button("📂").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.folder_path = path.display().to_string();
                                }
                            }
                        });
                        
                        // Encrpt / Decrpt Buttons
                        ui.add_space(64.0);
                        ui.horizontal(|ui| {
                            // encryption button
                            let encrypt_button = egui::Button::new("Encrypt Files")
                                .min_size(egui::Vec2::new(132.0, 24.0));
                            if ui.add(encrypt_button).clicked() {
                                self.encrpt();
                                // self.in_process = true;
                            }
                            ui.add_space(32.0);
                            // decrpytion button
                            let decrpyt_button = egui::Button::new("Decrypt Files")
                                .min_size(egui::Vec2::new(132.0, 24.0));
                            if ui.add(decrpyt_button).clicked() {
                                self.decrpt();
                                // self.in_process = true;
                            }
                        });

                    });
            });

        // Show popup window if encrypting or decrypting
        if self.in_process {
            egui::Window::new("🔄 Encrypting...")
                .collapsible(false)
                .resizable(false)
                .fixed_size(egui::vec2(300.0, 100.0))
                .show(ctx, |ui| {
                    ui.add(egui::ProgressBar::new(self.progress).show_percentage());

                    if self.progress >= 1.0 {
                        ui.label("✅ Done!");
                        println!("Encryption complete!");
                    } else {
                        ctx.request_repaint(); 
                    }
                });
            }
    }
}
