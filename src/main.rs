#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe;
use pomot::App;

const ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "pomot.png"));

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let icon = image::load_from_memory(ICON)
    .expect("Failed to open icon path.")
    .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let native_options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };
    
    eframe::run_native(
        "Pomodoro Timer",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
