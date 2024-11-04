pub mod gui {
    // Represent all types of windows as an enum
    //
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
        /// Our window name
        ///
        pub name: String,
        /// Our window type
        ///
        pub win_type: WindowType,
    }
    // Define our implementation of this struct, all we'll need is a construtor
    //
    impl Window {
        pub fn new(name: String, win_type: WindowType) -> Self {
            Self { name, win_type }
        }
    }
}

pub mod windows;
