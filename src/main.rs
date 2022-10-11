use eframe;
use pomot::App;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {

    let icon = image::open("pomot.png").expect("Failed to open icon path.").to_rgba8();
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
