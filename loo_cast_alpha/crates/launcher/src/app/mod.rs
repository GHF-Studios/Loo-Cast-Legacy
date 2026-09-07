//! Launcher application composition.

use eframe::egui;

use crate::state::LauncherState;
use crate::ui::LauncherUi;

const WINDOW_TITLE: &str = "Launcher";
const INITIAL_WINDOW_SIZE: [f32; 2] = [1400.0, 900.0];
const MIN_WINDOW_SIZE: [f32; 2] = [1000.0, 700.0];

pub fn run() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(INITIAL_WINDOW_SIZE)
            .with_min_inner_size(MIN_WINDOW_SIZE),
        ..Default::default()
    };

    eframe::run_native(WINDOW_TITLE, native_options, Box::new(|_creation_context| Ok(Box::new(LauncherApp::default()))))
}

#[derive(Default)]
struct LauncherApp {
    state: LauncherState,
}

impl eframe::App for LauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        LauncherUi::draw(ui, &mut self.state);
    }
}
