use super::{FunctionEnum, ModuleEnum};
use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use crate::iterators::module_iter::ModuleIterator;
use crate::pe::pe::{get_imports, get_imports_descriptor_from_name, get_pe_from_path};
use crate::process::Process;
use crate::to_rstr;
use egui::Ui;
use exe::pe::VecPE;
use exe::CCharString;

/// Create out custom TabContent object for this specific tab, in this case it will be for our
/// Imports view
///
#[derive(Clone)]
pub struct ImportsView {
    pub selected_module_enum: Option<ModuleEnum>,
    pub selected_module: Option<String>,
    pub selected_function_enum: Option<FunctionEnum>,
    pub selected_function: Option<String>,
    pub process_path: Option<String>,
    pub pe_file: Option<VecPE>,
    pub process: Process,
}

/// Form abstract link to TabContent
///
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
                // Store static height because any subsequent call to ui.available_width()
                // in dynamic settings will just continously blow itself up with any
                // changes
                let total_width = ui.available_width();
                let panel_width = total_width / 3.0;
                // Create three views, since I would've loved to use a table, we need more than one
                // input per column as we're going to need to iterate over all of its imports
                egui::SidePanel::left("modules_panel_imports_left")
                    .resizable(false)
                    .exact_width(panel_width)
                    .show_inside(ui, |ui| {
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

                                        // The actual selectable button in question...
                                        let button = ui.add_sized(
                                            [panel_width, 20.0],
                                            egui::SelectableLabel::new(is_selected, &mod_name_str),
                                        );

                                        // If it was clicked then store it
                                        if button.clicked() {
                                            self.selected_module_enum = Some(module_enum);
                                            self.selected_module = Some(mod_name_str);
                                        }
                                    }
                                }
                            });
                        });
                    });
                // This frame will show all the imported functions from selected module
                egui::SidePanel::right("modules_panel_imports_right")
                    .resizable(false)
                    .exact_width(panel_width)
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
                // Work on the next frame, this will show extra information as well as buttons for
                // the specific function
                egui::CentralPanel::default()
                    .frame(egui::Frame::none().inner_margin(egui::Margin {
                        left: ui.available_width() * 0.1,
                        right: ui.available_width() * 0.1,
                        top: 0.0,
                        bottom: 0.0,
                    }))
                    .show_inside(ui, |ui| {
                        // This will get messy fast since we're going to have to work with some
                        // extra parsing and actually reading it from the process itself.
                        if let Some(func_name) = &self.selected_function {
                            ui.label(func_name);
                            ui.add_space(5.0);

                            // Get import descriptor details about the current module and function
                            // we're dealing with
                            if let (Some(pe), Some(module)) = (&self.pe_file, &self.selected_module)
                            {
                                if let Ok(descriptor) =
                                    get_imports_descriptor_from_name(pe.clone(), module.clone())
                                {
                                    // Get the index of the current imported function
                                    match descriptor.get_imports(pe) {
                                        Ok(imports) => {
                                            if let Some(idx) = imports.iter().position(|import| {
                                                let import_name = match import {
                                                    exe::ImportData::Ordinal(ord) => {
                                                        ord.to_string()
                                                    }
                                                    exe::ImportData::ImportByName(name) => {
                                                        name.to_string()
                                                    }
                                                };
                                                import_name == *func_name
                                            }) {
                                                // Get thunks
                                                if let Ok(thunks) = descriptor.get_first_thunk(pe) {
                                                    if idx < thunks.len() {
                                                        let function_rva = descriptor.first_thunk.0
                                                            + (idx as u32 * 8);
                                                        let module_full = unsafe {
                                                            ModuleIterator::new(self.process.pid())
                                                                .unwrap()
                                                                .find(|new_module| {
                                                                    to_rstr!(new_module.szModule)
                                                                        .to_lowercase()
                                                                        == *module.to_lowercase()
                                                                })
                                                                .unwrap()
                                                        };
                                                        let func_address = unsafe {
                                                            self.process
                                                                .read::<u64>(
                                                                    module_full.modBaseAddr
                                                                        as usize
                                                                        + function_rva as usize,
                                                                )
                                                                .unwrap()
                                                        };
                                                        ui.label(format!("{:X}", func_address));
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
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
