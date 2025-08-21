#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use opencade_client::TemplateApp; // robust: refers to this package's lib target

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "OpenCade",
        native_options,
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    )
}
