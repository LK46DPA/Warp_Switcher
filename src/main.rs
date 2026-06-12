use eframe::egui;
use egui::vec2;
use std::process::Command;
use std::process::Stdio;
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cloudflare Warp Switcher",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    flag: bool,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Cloud Flare warp switcher ");
            if ui
                .add(egui::Button::new("On").min_size(vec2(120.0, 40.0)))
                .clicked()
            {
                let output = Command::new("warp-cli")
                    .arg("connect")
                    .output()
                    .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
                self.flag = true;
            }
            if ui
                .add(egui::Button::new("Off").min_size(vec2(120.0, 40.0)))
                .clicked()
            {
                let output = Command::new("warp-cli")
                    .arg("disconnect")
                    .output()
                    .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
                self.flag = false;
            }

            if self.flag == true {
                ui.heading("Connected");
            } else {
                ui.heading("Disconnected");
            }
        });
    }
}
