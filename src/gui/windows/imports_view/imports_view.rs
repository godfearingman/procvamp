use super::{FunctionEnum, ModuleEnum};
use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use crate::pe::pe::{get_imports, get_imports_descriptor_from_name, get_pe_from_path};
use egui::Ui;
use exe::pe::VecPE;
use exe::CCharString;

// Create out custom TabContent object for this specific tab, in this case it will be for our
// module view
//
#[derive(Clone)]
pub struct ImportsView {
    pub selected_module_enum: Option<ModuleEnum>,
    pub selected_module: Option<String>,
    pub selected_function_enum: Option<FunctionEnum>,
    pub selected_function: Option<String>,
    pub process_path: Option<String>,
    pub frame_width: Option<f32>,
    pub pe_file: Option<VecPE>,
}

// Form abstract link to TabContent
//
impl TabContent for ImportsView {
    /// Cursed ass code incoming not even gonna lie.
    fn ui(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                // Store the PE file that we'll be needing it in two different views
                if self.pe_file.is_none() {
                    self.pe_file = self
                        .process_path
                        .as_ref()
                        .map(|path| path)
                        .and_then(|path| match get_pe_from_path(path.clone()) {
                            Ok(pe) => Some(pe),
                            Err(_) => None,
                        });
                }
                // Create two views, since I would've loved to use a table, we need more than one
                // input per column as we're going to need to iterate over all of its imports
                egui::SidePanel::left("modules_panel_imports")
                    .resizable(true)
                    .default_width(ui.available_width())
                    .show_inside(ui, |ui| {
                        // Store static height because any subsequent call to ui.available_width()
                        // in dynamic settings will just continously blow itself up with any
                        // changes
                        if self.frame_width.is_none() {
                            self.frame_width = Some(ui.available_width() / 3.0);
                        }

                        // Create a list of all modules, once clicked it will store the selected in
                        // order for the imports to be displayed
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                // Iterate through every import and store its descriptor
                                if let Some(pe) = &self.pe_file {
                                    let imports = get_imports(&pe);
                                    for descriptor in imports.unwrap().descriptors {
                                        // Get module name
                                        let mod_name = descriptor.get_name(pe).unwrap();
                                        let mod_name_str =
                                            CCharString::as_str(mod_name).unwrap().to_string();
                                        // Create an enum member of all modules
                                        let module_enum = ModuleEnum::Title(mod_name_str.clone());
                                        // Check if it was selected
                                        let is_selected = self.selected_module_enum.as_ref()
                                            == Some(&module_enum);

                                        // Verify that we actually set the width (ngl we most
                                        // definitely have but we'll do this anyway)
                                        if let Some(width) = self.frame_width {
                                            // The actual selectable button in question...
                                            let button = ui.add_sized(
                                                [width, 20.0],
                                                egui::SelectableLabel::new(
                                                    is_selected,
                                                    &mod_name_str,
                                                ),
                                            );

                                            // If it was clicked then store it
                                            if button.clicked() {
                                                self.selected_module_enum = Some(module_enum);
                                                self.selected_module = Some(mod_name_str);
                                            }
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
                        self.selected_module
                            .as_ref()
                            .and_then(|module| Some((module, self.pe_file.as_ref())))
                            .and_then(|(module, pe)| {
                                let pe_clone = pe.unwrap().clone();
                                Some((
                                    pe_clone.clone(),
                                    get_imports_descriptor_from_name(pe_clone, module.to_string()),
                                ))
                            })
                            .map(|(pe, descriptor)| {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::LEFT),
                                        |ui| {
                                            for import in
                                                descriptor.unwrap().get_imports(&pe).unwrap()
                                            {
                                                // Store function enum for every imported function
                                                let func_name = match import {
                                                    exe::ImportData::Ordinal(ord) => {
                                                        ord.to_string()
                                                    }
                                                    exe::ImportData::ImportByName(name) => {
                                                        name.to_string()
                                                    }
                                                };
                                                let function_enum =
                                                    FunctionEnum::Title(func_name.clone());
                                                // Check if function is already selected
                                                let is_selected =
                                                    self.selected_function_enum.as_ref()
                                                        == Some(&function_enum);

                                                let button = ui.add_sized(
                                                    [ui.available_width(), 20.0],
                                                    egui::SelectableLabel::new(
                                                        is_selected,
                                                        func_name.clone(),
                                                    ),
                                                );

                                                // Check if clicked
                                                if button.clicked() {
                                                    self.selected_function_enum =
                                                        Some(function_enum);
                                                    self.selected_function = Some(func_name);
                                                }
                                            }
                                        },
                                    )
                                })
                            });
                    });
            });
    }

    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return "[>] Imports".to_string();
    }
}
