#![windows_subsystem = "windows"]

use eframe::egui;

mod classifier;
mod organizer;
mod gui;

fn main() -> eframe::Result<()> {
    // Configure eframe options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 450.0])
            .with_min_inner_size([400.0, 400.0])
            .with_title("ravsa"),
        ..Default::default()
    };

    eframe::run_native(
        "ravsa",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\arialbd.ttf") {
                fonts.font_data.insert("helvetica_bold".to_owned(), egui::FontData::from_owned(font_data));
                fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "helvetica_bold".to_owned());
            }
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(gui::OrganizadorApp::default()))
        }),
    )
}
