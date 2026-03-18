use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::fs;

use crate::organizer::{
    get_base_dir, escanear_archivos, hay_archivos_para_revertir,
    recolectar_archivos_para_revertir, limpiar_todas_las_carpetas_vacias, resolver_colision
};

#[derive(Default, PartialEq)]
enum AppState {
    #[default]
    Idle,
    Organizing,
    Reverting,
}

pub struct OrganizadorApp {
    base_dir: PathBuf,
    state: AppState,
    files_to_revert: usize,
    progress: f32,
    status_msg: String,
    detailed_msg: String,
    rx: Option<mpsc::Receiver<(f32, String, Option<String>)>>,
    log_entries: Vec<String>,
    current_height: f32,
}

impl Default for OrganizadorApp {
    fn default() -> Self {
        let dir = get_base_dir();
        let revert_count = hay_archivos_para_revertir(&dir);
        Self {
            base_dir: dir.clone(),
            state: AppState::Idle,
            files_to_revert: revert_count,
            progress: 0.0,
            status_msg: format!("Buscando PDFs en: {}", dir.display()),
            detailed_msg: String::new(),
            rx: None,
            log_entries: Vec::new(),
            current_height: 280.0,
        }
    }
}

impl OrganizadorApp {
    fn start_organize(&mut self) {
        self.state = AppState::Organizing;
        self.progress = 0.0;
        self.status_msg = "Calculando archivos a organizar...".to_string();
        self.detailed_msg = String::new();
        self.log_entries.clear();
        
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let base_dir = self.base_dir.clone();
        
        thread::spawn(move || {
            let trabajos = escanear_archivos(&base_dir);
            let total = trabajos.len();
            if total == 0 {
                let _ = tx.send((1.0, "No se encontraron nuevos archivos para procesar.".to_string(), None));
                return;
            }
            
            let mut folders_logged = std::collections::HashSet::new();
            
            for (i, (src, dest)) in trabajos.iter().enumerate() {
                if let Some(parent) = dest.parent() {
                    if !parent.exists() {
                        let _ = fs::create_dir_all(parent);
                    }
                    if let Ok(rel) = parent.strip_prefix(&base_dir) {
                        if let Some(first_comp) = rel.components().next() {
                            let rel_str = first_comp.as_os_str().to_string_lossy().to_string();
                            if !folders_logged.contains(&rel_str) {
                                folders_logged.insert(rel_str.clone());
                                let _ = tx.send((0.0, String::new(), Some(rel_str.clone())));
                            }
                        }
                    }
                }
                let _ = fs::rename(src, dest);
                
                let percent = (i as f32 + 1.0) / total as f32;
                let text = format!("Movido [{}/{}]", i + 1, total);
                let _ = tx.send((percent, text, None));
            }
            
            let _ = tx.send((1.0, format!("¡Organización completa! ({} movidos)", total), None));
        });
    }

    fn start_revert(&mut self) {
        self.state = AppState::Reverting;
        self.progress = 0.0;
        self.status_msg = "Calculando archivos a revertir...".to_string();
        self.detailed_msg = String::new();
        self.log_entries.clear();
        
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let base_dir = self.base_dir.clone();
        
        thread::spawn(move || {
            let archivos = recolectar_archivos_para_revertir(&base_dir);
            let total = archivos.len();
            if total == 0 {
                let _ = tx.send((1.0, "No hay archivos organizados para deshacer.".to_string(), None));
                return;
            }
            
            for (i, src) in archivos.iter().enumerate() {
                let filename = src.file_name().unwrap();
                let dest = resolver_colision(base_dir.join(filename));
                let _ = fs::rename(src, &dest);
                
                let percent = (i as f32 + 1.0) / total as f32;
                let text = format!("Revirtiendo [{}/{}]", i + 1, total);
                let _ = tx.send((percent, text, None));
            }
            
            let _ = tx.send((0.99, "Limpiando carpetas vacías...".to_string(), Some("🧹 Limpiando carpetas vacías...".to_string())));
            limpiar_todas_las_carpetas_vacias(&base_dir);
            let _ = tx.send((1.0, format!("¡Reversión completa! ({} movidos de vuelta)", total), None));
        });
    }
}

impl eframe::App for OrganizadorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Theme (Dark Blue Tkinter '#1A3C8A' equivalent)
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(26, 60, 138); // #1A3C8A
        visuals.override_text_color = Some(egui::Color32::WHITE);
        ctx.set_visuals(visuals);

        // Update from channel
        if let Some(rx) = &self.rx {
            let mut got_messages = false;
            for (p, msg, log) in rx.try_iter() {
                if p > 0.0 || !msg.is_empty() {
                    self.progress = p;
                    self.detailed_msg = msg;
                }
                if let Some(l) = log {
                    self.log_entries.push(l);
                }
                got_messages = true;
                if p >= 1.0 {
                    self.state = AppState::Idle;
                    self.files_to_revert = hay_archivos_para_revertir(&self.base_dir);
                    self.status_msg = if self.detailed_msg.is_empty() { "Operación completa".to_string() } else { self.detailed_msg.clone() };
                    self.detailed_msg.clear();
                }
            }
            if got_messages && self.state != AppState::Idle {
                ctx.request_repaint();
            }
        }

        // Dynamic Resize Control
        let desired_height = if self.log_entries.is_empty() { 280.0 } else { 450.0 };
        let desired_width = if self.log_entries.is_empty() { 400.0 } else { 550.0 };
        
        if (self.current_height - desired_height).abs() > 1.0 {
            self.current_height = desired_height;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(desired_width, desired_height)));
        }

        let panel_frame = egui::Frame::none()
            .inner_margin(0.0)
            .fill(egui::Color32::from_rgb(26, 60, 138));

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui: &mut egui::Ui| {
            // PARTE 1: Controles de arriba (con margen)
            egui::Frame::none()
                .inner_margin(egui::Margin::symmetric(20.0, 10.0))
                .show(ui, |ui: &mut egui::Ui| {
                    ui.vertical_centered(|ui: &mut egui::Ui| {
                        ui.add_space(10.0);
                        ui.heading(egui::RichText::new("ravsa").size(32.0).color(egui::Color32::WHITE));
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Organizador Ejecutivo de Archivos").size(14.0).color(egui::Color32::WHITE));
                        ui.add_space(20.0);
                        
                        let progress_bar = egui::ProgressBar::new(self.progress)
                            .show_percentage()
                            .animate(self.state != AppState::Idle);
                        ui.add(progress_bar);
                        
                        ui.add_space(10.0);
                        let status_color = if self.progress >= 1.0 { egui::Color32::from_rgb(100, 255, 100) } else { egui::Color32::WHITE };
                        ui.label(egui::RichText::new(&self.status_msg).color(status_color).size(12.0));
                        ui.label(egui::RichText::new(&self.detailed_msg).size(12.0));
                        
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui: &mut egui::Ui| {
                            let w = ui.available_width();
                            ui.add_space((w - 340.0) / 2.0); // Centrado balanceado
                            
                            let organize_btn = ui.add_sized([160.0, 45.0], egui::Button::new(egui::RichText::new("Organizar Archivos").size(15.0).color(egui::Color32::WHITE)).fill(egui::Color32::from_rgb(40, 100, 200)));
                            if organize_btn.clicked() && self.state == AppState::Idle {
                                self.start_organize();
                            }
                            
                            ui.add_space(20.0);
                            
                            let btn_fill = if self.files_to_revert == 0 { egui::Color32::from_rgb(120, 120, 120) } else { egui::Color32::from_rgb(200, 60, 60) };
                            let revert_btn = ui.add_sized([160.0, 45.0], egui::Button::new(egui::RichText::new("↩ REVERTIR").size(15.0).color(egui::Color32::WHITE)).fill(btn_fill));
                            if revert_btn.clicked() && self.state == AppState::Idle && self.files_to_revert > 0 {
                                self.start_revert();
                            }
                        });
                    });
                });

            // PARTE 2: Logs (sin margen, expandible)
            if !self.log_entries.is_empty() {
                let title = if self.state == AppState::Idle { "Carpetas afectadas:" } else { "Registro de actividad:" };
                
                ui.separator();
                ui.add_space(5.0);
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new(title).strong().size(14.0).color(egui::Color32::WHITE));
                });
                ui.add_space(5.0);
                
                egui::Frame::canvas(&ui.style())
                    .fill(egui::Color32::from_rgb(20, 45, 105))
                    .show(ui, |ui: &mut egui::Ui| {
                        // Absorbe todo el espacio restante
                        ui.set_height(ui.available_height());
                        
                        egui::ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .auto_shrink([false, false])
                            .show(ui, |ui: &mut egui::Ui| {
                                for log in &self.log_entries {
                                    ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.add_space(20.0);
                                        ui.label(egui::RichText::new(log).size(14.0).color(egui::Color32::WHITE));
                                    });
                                    ui.add_space(3.0);
                                }
                            });
                    });
            }
        });
    }
}
