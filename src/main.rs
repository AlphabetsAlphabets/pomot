use eframe;
use pomot::App;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pomodoro Timer",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
