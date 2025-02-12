use crate::gui::main::debug::debug::DebugView;
use eframe::{egui, NativeOptions};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

/// Define views
///
enum View {
    Attach,
    Debug,
}

struct MyApp {
    current_view: View,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_view {
            View::Attach => {}
            View::Debug => {}
        }
    }
}

pub fn run_gui() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "procvamp ^-^",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
