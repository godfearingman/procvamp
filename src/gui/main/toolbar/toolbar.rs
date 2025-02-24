use crate::gui::gui::Window;
use crate::gui::gui::WindowType;
use crate::gui::main::Tab;
use crate::gui::windows::disassembly_view::disassembly_view::DisassemblyView;
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
            let _ = ui.button("Function");
            let _ = ui.button("Graph");
            let _ = ui.button("Scanner");
            let _ = ui.button("Scanner Results");
        });
        ui.menu_button("Settings", |_ui| {});
    });

    new_window
}
