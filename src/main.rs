use encryptor::gui::MainWindow;
use eframe;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.centered = true;
    options.viewport.resizable = Some(false);
    options.viewport.max_inner_size = Some(eframe::egui::vec2(480.0, 360.0));
    
    eframe::run_native(
        "File Encryptor",
        options,
        Box::new(|_cc| Ok(Box::new(MainWindow::default()))),
    ).expect("failed to open the app");
}
