use crate::gui::main::attach::attach::AttachView;
use crate::gui::main::debug::debug::DebugView;
use eframe::{egui, NativeOptions};

/// Define views
///
enum View {
    Attach(AttachView),
    Debug(DebugView),
}

struct MyApp {
    current_view: View,
}

impl Default for MyApp {
    fn default() -> Self {
        unsafe {
            Self {
                current_view: View::Attach(AttachView::new().unwrap()),
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.current_view {
            View::Attach(attach_view) => {
                if let Some(proc) = attach_view.show(ctx) {
                    self.current_view = View::Debug(DebugView::new(proc));
                }
            }
            View::Debug(dbg_view) => dbg_view.show(ctx),
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
