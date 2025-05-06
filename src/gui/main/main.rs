use crate::gui::main::attach::attach::AttachView;
use crate::gui::main::debug::debug::DebugView;
use eframe::egui;
use std::sync::Arc;

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
        // Show menu bar
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
    let icon_data =
        eframe::icon_data::from_png_bytes(include_bytes!("proc_vamp_logo.ico")).unwrap();
    let mut options = eframe::NativeOptions::default();
    options.viewport.icon = Some(Arc::new(icon_data));

    eframe::run_native(
        "procvamp",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
