use egui::{Color32, RichText};
use std::{time::Duration};

enum Warning {
    None,
    Message(String),
}

enum Screen {
    Start,
    Rest,
    Working,
    Finish,
}

fn start_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Pomodoro Timer");
            ui.hyperlink_to(
                "Made by AlphabetsAlphavets",
                "https://github.com/AlphabetsAlphabets",
            );
            
            match &app.warning {
                Warning::None => ui.label("\n\n\n"),
                Warning::Message(msg) => {
                    let msg = format!("\n{}\n", msg);
                    ui.label(msg)
                }
            };

            ui.label("Work");

            // Notes
            // This used to be what I wrote, the reason this doesn't work is because
            // the string expression ends up returning it's own string that is separate
            // from `app.work` which is why the text won't update.
            // ui.text_edit_singleline(&mut work.to_string())
            //   .on_hover_text("Time you will work for each session in minutes.");
            ui.text_edit_singleline(&mut app.work)
                .on_hover_text("Time you will work for each session in minutes.");

            ui.label("Rest");
            ui.text_edit_singleline(&mut app.rest)
                .on_hover_text("Time you will rest between each session in minutes.");

            ui.label("Session");
            ui.text_edit_singleline(&mut app.sessions)
                .on_hover_text("The number of sessions you'll work for.");

            ui.label("");

            if ui
                .button(RichText::new("Start").color(Color32::LIGHT_BLUE))
                .clicked()
            {
                let wrk: i16 = app.work.parse().unwrap_or(-1);
                let rst: i16 = app.rest.parse().unwrap_or(-1);
                let sess: i16 = app.sessions.parse().unwrap_or(-1);

                println!(
                    "Work: {} minutes\nRest: {} minutes\nSessions: {}",
                    wrk, rst, sess
                );

                if wrk < 0 || rst < 0 || sess < 0 {
                    app.work = "".to_string();
                    app.rest = "".to_string();
                    app.sessions = "".to_string();
                    app.warning = Warning::Message("The fields can't be empty, they also can't contain letters, if for some reason you tried to enter a negative number it didn't work either :3.".to_string());
                } else if wrk == 0 || rst == 0 || sess == 0 {
                    app.work = "".to_string();
                    app.rest = "".to_string();
                    app.sessions = "".to_string();
                    app.warning = Warning::Message("Work, rest or sessions can't be empty or 0 buddy :3".to_string());
                } else {
                    app.warning = Warning::None;
                    app.screen = Screen::Working;
                }

            }
        });
    });
}

fn work_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Pomodoro Timer");
            ui.hyperlink_to(
                "Made by AlphabetsAlphabets",
                "https://github.com/AlphabetsAlphabets",
            );

            ui.label("\n\n\n");

            if ui
                .button(RichText::new("Pause").color(Color32::LIGHT_BLUE))
                .clicked()
            {
                println!("Paused.");
            }

            if ui
                .button(RichText::new("Stop")
                .color(Color32::LIGHT_BLUE))
                .on_hover_text("Stops the pomodoro sesssion and brings you back to the start screen.")
                .clicked()
            {
                app.screen = Screen::Start;
                println!("Stopped.");
            }
        });
    });
}

fn rest_screen() {}

fn finish_screen() {}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    work: String,
    rest: String,
    sessions: String,

    #[serde(skip)]
    warning: Warning,
    #[serde(skip)]
    screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            warning: Warning::None,
            screen: Screen::Start,
            work: "25".to_string(),
            rest: "10".to_string(),
            sessions: "4".to_string(),
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
            warning,
            sessions,
            work,
            rest,
            screen,
        } = self;

        match screen {
            Screen::Start => start_screen(self, ctx, _frame),
            Screen::Working => work_screen(self, ctx, _frame),
            Screen::Rest => rest_screen(),
            Screen::Finish => finish_screen(),
        }
    }
}
