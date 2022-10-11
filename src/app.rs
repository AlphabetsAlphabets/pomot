use egui::{Color32, RichText};
use std::{time::{Duration, Instant}};

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
                None => ui.label("\n\n\n"),
                Some(msg) => {
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
                    app.warning = Some("The fields can't be empty, they also can't contain letters, if for some reason you tried to enter a negative number it didn't work either :3.".to_string());
                } else if wrk == 0 || rst == 0 || sess == 0 {
                    app.work = "".to_string();
                    app.rest = "".to_string();
                    app.sessions = "".to_string();
                    app.warning = Some("Work, rest or sessions can't be empty or 0 buddy :3".to_string());
                } else {
                    // Convert input to duration

                    let work_time: u64 = app.work.parse().unwrap();
                    let rest_time: u64 = app.rest.parse().unwrap();
                    let session_count: i8 = app.sessions.parse().unwrap();

                    let work_time = Duration::from_secs(work_time * 60);
                    let rest_time = Duration::from_secs(rest_time * 60);

                    // Instant is started here because the value doesn't get reset every frame.
                    // As opposed to setting it in `work_screen`.
                    app.now = Some(Instant::now());

                    app.work_time = work_time;
                    app.rest_time = rest_time;
                    app.session_count = session_count;

                    app.warning = None;
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
                "Made by AlphabetsAlphavets",
                "https://github.com/AlphabetsAlphabets",
            );

            ui.label("\n\n\n");

            ui.label("Close the study door, for the work begins.");

            let elapsed = app.now.unwrap().elapsed().as_secs();
            let total_time = app.work_time.as_secs();
            let time_left = total_time.saturating_sub(elapsed);
            let time_left: String = if time_left >= 60 {
                format!("{}m", (time_left / 60))
            } else {
                format!("{}s", (time_left))
            };

            // Updates only show as long as there is an action made
            // Which includes typing, moving the mouse, etc.
            // The timer still sticks, it just won't update visually. 
            // The fix is to request egui to repaint the UI.
            ui.ctx().request_repaint();
            ui.heading(time_left.clone());

            ui.label("\n\n\n");
            ui.columns(2, |columns| {
                let pause = columns[0]
                    .button(RichText::new("Pause")
                    .color(Color32::LIGHT_BLUE));

                if pause.clicked() {
                    println!("Paused!");
                }

                let stop = columns[1]
                    .button(RichText::new("Stop")
                    .color(Color32::LIGHT_BLUE));

                if stop.clicked() {
                    app.screen = Screen::Start;
                }
            });

            if time_left == "0s" {
                app.screen = Screen::Rest;
                // `Some(Instant::now())` is very important. Without this the screens will very quickly
                // bounce between work and rest screen. Because without it `app.now` remains the same.
                // When transitioning to the `rest_screen` function, it does the same math. WHich means that
                // `time_left` is still at "0s" the if statement would turn true and `app.screen = Screen::Working`.
                // This happens in `work_screen` too. Resulting in the very fast back and forth switching.
                app.now = Some(Instant::now());
                return;
            }
        });
    });
}

fn rest_screen(app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Pomodoro Timer");
            ui.hyperlink_to(
                "Made by AlphabetsAlphavets",
                "https://github.com/AlphabetsAlphabets",
            );

            ui.label("\n\n\n");

            ui.label("Rest time.");

            let elapsed = app.now.unwrap().elapsed().as_secs();
            let total_time = app.rest_time.as_secs();
            let time_left = total_time.saturating_sub(elapsed);
            let time_left: String = if time_left >= 60 {
                format!("{}m", (time_left / 60))
            } else {
                format!("{}s", (time_left))
            };

            // Updates only show as long as there is an action made
            // Which includes typing, moving the mouse, etc.
            // The timer still sticks, it just won't update visually. 
            // The fix is to request egui to repaint the UI.
            ui.ctx().request_repaint();
            ui.heading(time_left.clone());

            ui.label("\n\n\n");
            ui.columns(2, |columns| {
                let pause = columns[0]
                    .button(RichText::new("Pause")
                    .color(Color32::LIGHT_BLUE));

                if pause.clicked() {
                    println!("Paused!");
                }

                let stop = columns[1]
                    .button(RichText::new("Stop")
                    .color(Color32::LIGHT_BLUE));

                if stop.clicked() {
                    app.screen = Screen::Start;
                }
            });

            if time_left == "0s" {
                app.screen = Screen::Working;
                return;
            }
        });
    });
}

fn finish_screen() {}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    work: String,
    rest: String,
    sessions: String,

    #[serde(skip)]
    now: Option<Instant>,
    #[serde(skip)]
    work_time: Duration,
    #[serde(skip)]
    rest_time: Duration,
    #[serde(skip)]
    warning: Option<String>,
    #[serde(skip)]
    session_count: i8,
    #[serde(skip)]
    screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            now: None,
            warning: None,
            screen: Screen::Start,
            work: "25".to_string(),
            rest: "10".to_string(),
            sessions: "4".to_string(),
            work_time: Duration::from_secs(1 * 60),
            rest_time: Duration::from_secs(1 * 60),
            session_count: 4,
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
        match self.screen {
            Screen::Start => start_screen(self, ctx, _frame),
            Screen::Working => work_screen(self, ctx, _frame),
            Screen::Rest => rest_screen(self, ctx, _frame),
            Screen::Finish => finish_screen(),
        }
    }
}
