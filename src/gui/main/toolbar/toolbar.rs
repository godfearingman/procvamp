use crate::gui::gui::Window;
use crate::gui::gui::WindowType;
use crate::gui::main::Tab;
use crate::gui::windows::disassembly_view::disassembly_view::DisassemblyView;
use crate::gui::windows::imports_view::imports_view::ImportsView;
use crate::gui::windows::module_view::module_view::ModuleView;
use crate::memory::process::process::Process;
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
                // Get all loaded modules again and send it over
                let process_modules = unsafe { process.get_modules().unwrap() };
                new_window = Some(Window::new(
                    WindowType::ImportsView,
                    Tab::Imports(ImportsView {
                        modules: process_modules,
                        selected_module_enum: None,
                        selected_module: None,
                        selected_function: None,
                        frame_width: None,
                    }),
                ));
            }
            let _ = ui.button("Allocations");
            let _ = ui.button("Function");
            let _ = ui.button("Graph");
            let _ = ui.button("Scanner");
            let _ = ui.button("Scanner Results");
        });
        ui.menu_button("Settings", |_ui| {});
    });

    new_window
}
