use crate::gui::gui::TabContent;
use crate::gui::gui::Window;
use crate::gui::gui::WindowType;
use crate::gui::windows::disassembly_view::disassembly_view::DisassemblyView;
use crate::gui::windows::ActiveWindows;
use eframe::{egui, NativeOptions};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

// Another enum which will map entries to actual views
//
pub enum Tab {
    Disassembly(DisassemblyView),
}

// Handle abstract tab system, here we'll just match the enum members to what is currently
// active
//
impl Tab {
    fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Tab::Disassembly(view) => view.ui(ui),
        }
    }
    fn title(&self) -> String {
        match self {
            Tab::Disassembly(view) => view.title(),
        }
    }
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Window;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.win_content.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.win_content.ui(ui);
    }
}

struct MyApp {
    tree: DockState<Window>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut windows_manager = ActiveWindows::default();
        let disasm_win = Window::new(
            WindowType::DisassemblyView,
            Box::new(DisassemblyView {
                address_start: 0xDEADCAFE,
            }),
        );
        let disasm_win2 = Window::new(
            WindowType::DisassemblyView,
            Box::new(DisassemblyView {
                address_start: 0xDEADBEEF,
            }),
        );

        windows_manager.add_tab(disasm_win);
        windows_manager.add_tab(disasm_win2);

        let tree = DockState::new(windows_manager.tabs);

        Self { tree }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {});
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
