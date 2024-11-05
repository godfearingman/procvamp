pub mod gui {
    use egui::Ui;
    // Represent all types of windows as an enum
    //
    #[derive(Debug, Clone, PartialEq)]
    pub enum WindowType {
        DisassemblyView,
        FunctionView,
        ScannerView,
        ScannerResults,
        GraphView,
    }
    // Basic struct for defining windows and what type they are
    //
    pub struct Window {
        // Our window type
        //
        pub win_type: WindowType,
        // Our window content
        //
        pub win_content: Box<dyn TabContent>,
    }
    // Define our implementation of this struct, all we'll need is a construtor
    //
    impl Window {
        pub fn new(win_type: WindowType, win_content: Box<dyn TabContent>) -> Self {
            Self {
                win_type,
                win_content,
            }
        }
    }
    // Define our trait for an abstract type docking system
    //
    pub trait TabContent {
        // This will be where the actual body of what the tab will hold is displayed
        //
        fn ui(&mut self, ui: &mut Ui);
        // This will be where the title of the tab will be displayed from
        //
        fn title(&self) -> String;
    }
}

pub mod main;
pub mod windows;
