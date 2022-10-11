// This is used to hide the console from popping up on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe;
use pomot::App;

/// `env!("CARGO_MANIFEST_DIR)` will locate for the directory of the manifest. Which is also the crate root.
/// `concat!` just merges the file path obtained from the `env!` call and sticks "/pomot.png" to it.
/// `include_bytes!` will read the data in the file `pomot.png`. This makes it so the image data will
/// remain in the binary itself.
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
