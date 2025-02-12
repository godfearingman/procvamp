use crate::gui::gui::TabContent;
use crate::gui::gui::Window;
use crate::gui::gui::WindowType;
use crate::gui::windows::disassembly_view::disassembly_view::DisassemblyView;
use crate::gui::windows::function_view::function_view::FunctionView;
use crate::gui::windows::ActiveWindows;
use eframe::{egui, NativeOptions};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

// Another enum which will map entries to actual views
//
#[derive(Clone)]
pub enum Tab {
    Disassembly(DisassemblyView),
    Function(FunctionView),
}

// Handle abstract tab system, here we'll just match the enum members to what is currently
// active
//
impl TabContent for Tab {
    fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Tab::Disassembly(view) => view.ui(ui),
            Tab::Function(view) => view.ui(ui),
        }
    }
    fn title(&self) -> String {
        match self {
            Tab::Disassembly(view) => view.title(),
            Tab::Function(view) => view.title(),
        }
    }
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Window<Tab>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.win_content.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.win_content.ui(ui);
    }
}

pub struct DebugView {
    tree: DockState<Window<Tab>>,
}

impl Default for DebugView {
    fn default() -> Self {
        // Setup our initial view of the entire window
        //
        let mut windows_manager = ActiveWindows::default();

        // Setup our dummy function view
        //
        let mut func_view = FunctionView::new();
        func_view.set(format!("fn_{:016X}", 0x1000), 0x1000);
        func_view.set(format!("fn_{:016X}", 0x1001), 0x1001);
        func_view.set(format!("fn_{:016X}", 0x1002), 0x1002);

        let func_win = Window::new(WindowType::FunctionView, Tab::Function(func_view));

        // Testing to implement some basic boiler tabs
        //
        let disasm_win = Window::new(
            WindowType::DisassemblyView,
            Tab::Disassembly(DisassemblyView {
                address_start: 0xDEADCAFE,
            }),
        );
        let disasm_win2 = Window::new(
            WindowType::DisassemblyView,
            Tab::Disassembly(DisassemblyView {
                address_start: 0xDEADBEEF,
            }),
        );
        let disasm_win3 = Window::new(
            WindowType::DisassemblyView,
            Tab::Disassembly(DisassemblyView {
                address_start: 0xDEADC0FFEE,
            }),
        );

        // Add two tabs to the main tree
        //
        windows_manager.add_tab(disasm_win);
        windows_manager.add_tab(disasm_win2);
        windows_manager.add_tab(disasm_win3);
        windows_manager.add_tab(func_win);

        let mut tree = DockState::new(windows_manager.tabs.clone());

        // Now add the same tabs onto a different view
        //
        let [_, _] = tree.main_surface_mut().split_left(
            NodeIndex::root(),
            0.35,
            windows_manager.tabs.clone(),
        );

        // Now add same tabs again onto a bottom view
        //
        let [_, _] = tree.main_surface_mut().split_below(
            NodeIndex::root(),
            0.6,
            windows_manager.tabs.clone(),
        );

        Self { tree }
    }
}

impl DebugView {
    pub fn show(&mut self, ctx: &egui::Context) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {});
    }
}
