pub mod attach;
pub mod debug;
pub mod main;
pub mod toolbar;
use egui::Color32;

use crate::gui::gui::TabContent;
use crate::gui::gui::Window;
use crate::gui::windows::disassembly_view::disassembly_view::DisassemblyView;
use crate::gui::windows::function_view::function_view::FunctionView;
use crate::gui::windows::imports_view::imports_view::ImportsView;
use crate::gui::windows::module_view::module_view::ModuleView;

// Our struct used for theme colours
//
pub struct ThemeColours {
    pub primary: Color32,
    pub secondary: Color32,
    pub background_dark: Color32,
    pub text_muted: Color32,
    pub highlight: Color32,
}

/// Define some colours
///

/// Start off with dark theme
///
pub const DARK_THEME: ThemeColours = ThemeColours {
    // Deep blood red
    primary: Color32::from_rgb(140, 0, 0),
    // Rich purple
    secondary: Color32::from_rgb(83, 53, 74),
    // Darker blood red for backgrounds
    background_dark: Color32::from_rgb(20, 0, 0),
    // Desaturated blood red for text
    text_muted: Color32::from_rgb(171, 103, 103),
    // Brighter blood red for highlights
    highlight: Color32::from_rgb(196, 27, 27),
};

// Another enum which will map entries to actual views
//
#[derive(Clone)]
pub enum Tab {
    Disassembly(DisassemblyView),
    Function(FunctionView),
    Module(ModuleView),
    Imports(ImportsView),
}

// Handle abstract tab system, here we'll just match the enum members to what is currently
// active
//
impl TabContent for Tab {
    fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Tab::Disassembly(view) => view.ui(ui),
            Tab::Function(view) => view.ui(ui),
            Tab::Module(view) => view.ui(ui),
            Tab::Imports(view) => view.ui(ui),
        }
    }
    fn title(&self) -> String {
        match self {
            Tab::Disassembly(view) => view.title(),
            Tab::Function(view) => view.title(),
            Tab::Module(view) => view.title(),
            Tab::Imports(view) => view.title(),
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
