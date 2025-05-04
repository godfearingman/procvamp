use crate::gui::gui::Window;
use crate::gui::gui::WindowType;
use crate::gui::main::Tab;
use crate::gui::windows::allocation_view::allocation_view::AllocationView;
use crate::gui::windows::disassembly_view::disassembly_view::DisassemblyView;
use crate::gui::windows::imports_view::imports_view::ImportsView;
use crate::gui::windows::module_view::module_view::ModuleView;
use crate::gui::windows::scanner_view::scanner_view::ScanType;
use crate::gui::windows::scanner_view::scanner_view::ScannerView;
use crate::gui::windows::scanner_view::scanner_view::ValueType;
use crate::iterators::allocation_iter::Allocation;
use crate::memory::process::process::Process;
use crate::to_rstr;
use eframe::egui;

/// Realistically we don't need any function other than show_bar so we won't create a struct but
/// rather just a standalone function that will be called in gui/main.rs in order to display the
/// menu bar
///
pub fn show_bar(ui: &mut egui::Ui, process: &mut Process) -> Option<Window<Tab>> {
    // Store return vector
    let mut new_window = None;
    // Create the menu frame
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |_ui| {});
        ui.menu_button("Views", |ui| {
            // Handle all views, if they click one of these buttons then we want to launch that tab
            // (aka just return it)
            let disassembly_button = ui.button("Disassembly");
            if disassembly_button.clicked() {
                // Create the view and set it to the start of the process
                let process_start = unsafe { process.base().unwrap() };

                new_window = Some(Window::new(
                    WindowType::DisassemblyView,
                    Tab::Disassembly(DisassemblyView {
                        address_start: process_start,
                        process: process.clone(),
                        bytes: Vec::new(),
                        instructions: Vec::new(),
                        bytes_read: 0,
                    }),
                ));
            }
            let module_button = ui.button("Modules");
            if module_button.clicked() {
                // Get all loaded modules and send it over instead of sending over the entire
                // process struct
                let process_modules = unsafe { process.get_modules().unwrap() };
                new_window = Some(Window::new(
                    WindowType::ModuleView,
                    Tab::Module(ModuleView {
                        modules: process_modules,
                        selected_module: None,
                        selected_module_enum: None,
                    }),
                ));
            }
            let imports_button = ui.button("Imports");
            if imports_button.clicked() {
                let process_modules = unsafe { process.get_modules().unwrap() };
                let process_path = process_modules
                    .iter()
                    .find(|module| to_rstr!(module.szModule) == process.name())
                    .map(|module| to_rstr!(module.szExePath))
                    .unwrap();

                new_window = Some(Window::new(
                    WindowType::ImportsView,
                    Tab::Imports(ImportsView {
                        selected_module_enum: None,
                        selected_module: None,
                        selected_function: None,
                        selected_function_enum: None,
                        process_path: Some(process_path),
                        pe_file: None,
                        process: process.clone(),
                    }),
                ));
            }
            let allocation_button = ui.button("Allocations");
            if allocation_button.clicked() {
                // get all allocations and send it over instead of sending over the entire process
                // struct
                let stored_allocs = unsafe { Allocation::new(process).unwrap().collect() };
                new_window = Some(Window::new(
                    WindowType::AllocationView,
                    Tab::Allocations(AllocationView {
                        selected_allocation_enum: None,
                        selected_allocation: None,
                        allocations: stored_allocs,
                    }),
                ));
            }
            let _ = ui.button("Function");
            let _ = ui.button("Graph");
            let scanner_button = ui.button("Scanner");
            if scanner_button.clicked() {
                let example_results = Vec::new();

                new_window = Some(Window::new(
                    WindowType::ScannerView,
                    Tab::Scanner(ScannerView {
                        process: process.clone(),
                        is_hex: false,
                        scan_type: ScanType::default(),
                        value_type: ValueType::default(),
                        value: String::new(),
                        fast_scan: true,
                        results: example_results,
                    }),
                ));
            }
            let _ = ui.button("Scanner Results");
        });
        ui.menu_button("Settings", |_ui| {});
    });

    new_window
}
