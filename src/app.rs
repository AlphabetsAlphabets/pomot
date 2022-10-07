use egui::{Color32, RichText};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    work: u16,
    rest: u8,
    sessions: u8,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            work: 25,
            rest: 5,
            sessions: 4,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            sessions,
            work,
            rest,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Pomodoro Timer");
                ui.hyperlink_to(
                    "Made by AlphabetsAlphavets",
                    "https://github.com/AlphabetsAlphabets",
                );
                ui.label("\n\n\n");

                ui.label("Work");
                ui.text_edit_singleline(&mut work.to_string())
                    .on_hover_text("Time you will work for each session in minutes.");

                ui.label("Rest");
                ui.text_edit_singleline(&mut rest.to_string())
                    .on_hover_text("Time you will rest for between session in minutes.");

                ui.label("Session");
                ui.text_edit_singleline(&mut sessions.to_string())
                    .on_hover_text("Number of session you plan to work for.");

                ui.label("");

                if ui
                    .button(RichText::new("Start").color(Color32::LIGHT_BLUE))
                    .clicked()
                {
                    println!("Hello!");
                }
            });
        });
    }
}
