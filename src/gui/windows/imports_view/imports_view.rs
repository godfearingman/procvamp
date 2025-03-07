use super::{FunctionEnum, ModuleEnum};
use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use crate::to_rstr;
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;

// Create out custom TabContent object for this specific tab, in this case it will be for our
// module view
//
#[derive(Clone)]
pub struct ImportsView {
    pub modules: Vec<MODULEENTRY32>,
    pub selected_module_enum: Option<ModuleEnum>,
    pub selected_module: Option<MODULEENTRY32>,
    pub selected_function: Option<FunctionEnum>,
    pub frame_width: Option<f32>,
}

// Form abstract link to TabContent
//
impl TabContent for ImportsView {
    fn ui(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                // Create two views, since I would've loved to use a table, we need more than one
                // input per column as we're going to need to iterate over all of its imports
                egui::SidePanel::left("modules_panel_imports")
                    .resizable(true)
                    .default_width(ui.available_width())
                    .show_inside(ui, |ui| {
                        // Store static height because any subsequent call to ui.available_width()
                        // in dynamic settings will just continously blow itself up with any
                        // changes
                        if None == self.frame_width {
                            self.frame_width = Some(ui.available_width() / 3.0);
                        }

                        // Create a list of all modules, once clicked it will store the selected in
                        // order for the imports to be displayed
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                for module in &self.modules {
                                    // Create an enum member of all modules
                                    let module_enum = ModuleEnum::Title(to_rstr!(module.szModule));
                                    // Check if it was selected
                                    let is_selected =
                                        self.selected_module_enum.as_ref() == Some(&module_enum);

                                    // Verify that we actually set the width (ngl we most
                                    // definitely have but we'll do this anyway)
                                    if let Some(width) = self.frame_width {
                                        // The actual selectable button in question...
                                        let button = ui.add_sized(
                                            [width, 20.0],
                                            egui::SelectableLabel::new(
                                                is_selected,
                                                to_rstr!(module.szModule),
                                            ),
                                        );

                                        // If it was clicked then store it
                                        if button.clicked() {
                                            self.selected_module_enum = Some(module_enum);
                                            self.selected_module = Some(*module);
                                        }
                                    }
                                }
                            });
                        });
                    });
                // Work on the next frame, this will show the actual imports of the module
                egui::CentralPanel::default()
                    .frame(egui::Frame::none().inner_margin(egui::Margin {
                        left: ui.available_width() * 0.1, // Add left margin for spacing
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    }))
                    .show_inside(ui, |ui| {
                        if let Some(_module) = self.selected_module {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    for _i in 1..256 {
                                        ui.label("fart");
                                    }
                                })
                            });
                        }
                    });
            });
    }

    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return "[>] Imports".to_string();
    }
}
