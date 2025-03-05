pub mod gui {
    use crate::memory::breakpoint::breakpoint::{Breakpoint, BreakpointState};
    use egui::{Color32, Response, Sense, Shape, Stroke, Ui};
    // Represent all types of windows as an enum
    //
    #[derive(Debug, Clone, PartialEq)]
    pub enum WindowType {
        DisassemblyView,
        FunctionView,
        ScannerView,
        ScannerResults,
        GraphView,
        ModuleView,
        AllocationView,
        ImportsView,
    }
    // Basic struct for defining windows and what type they are
    //
    #[derive(Debug, Clone)]
    pub struct Window<T: TabContent> {
        // Our window type
        //
        pub win_type: WindowType,
        // Our window content
        //
        pub win_content: T,
    }
    // Define our implementation of this struct, all we'll need is a construtor
    //
    impl<T: TabContent> Window<T> {
        pub fn new(win_type: WindowType, win_content: T) -> Self {
            Self {
                win_type,
                win_content,
            }
        }
    }
    // Define our trait for an abstract type docking system
    //
    pub trait TabContent: Clone {
        // This will be where the actual body of what the tab will hold is displayed
        //
        fn ui(&mut self, ui: &mut Ui);
        // This will be where the title of the tab will be displayed from
        //
        fn title(&self) -> String;
    }
    // Define a custom ui widget that we'll use for breakpointing
    //
    pub fn selectable_circle(ui: &mut Ui, colour: Color32) -> Response {
        // Define our widget size and allocate the space required within the ui
        //
        let widget_size = ui.spacing().interact_size.y;
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::new(widget_size, widget_size), Sense::click());

        // Create circle widget
        //
        let circle = Shape::circle_filled(rect.center(), rect.height() / 2.7, colour);

        // Add to painting board
        //
        ui.painter().add(circle);

        // Create outline
        //
        let circle_outline = Shape::circle_stroke(
            rect.center(),
            rect.height() / 2.8,
            Stroke::new(1.0, Color32::DARK_GRAY),
        );

        // Add to painting board
        //
        ui.painter().add(circle_outline);

        response
    }
    // Define a function for creating a custom bp icon near addresses in diassembly views
    //
    pub fn selectable_bp(ui: &mut Ui, bp: Option<Breakpoint>) -> Response {
        match bp {
            Some(breakpoint) => match breakpoint.bp_state {
                BreakpointState::Enabled => selectable_circle(ui, Color32::RED),
                BreakpointState::Disabled => selectable_circle(ui, Color32::DARK_RED),
            },
            None => selectable_circle(ui, Color32::DARK_GRAY),
        }
    }
}

pub mod main;
pub mod windows;
