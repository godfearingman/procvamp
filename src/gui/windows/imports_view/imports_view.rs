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
    pub selected_module: Option<ModuleEnum>,
    pub selected_function: Option<FunctionEnum>,
}

// Form abstract link to TabContent
//
impl TabContent for ImportsView {
    fn ui(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                // We don't know what the button height will be for every client so we want
                // to make sure the spacing is directly retreived from egui so we can
                // adjust our rows accordingly
                let button_height = ui.spacing().interact_size.y;

                TableBuilder::new(ui)
                    .cell_layout(egui::Layout::top_down(egui::Align::Center))
                    .column(Column::exact(250.0).resizable(false))
                    .column(Column::remainder())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Loaded modules");
                        });
                        header.col(|ui| {
                            ui.heading("Imported functions");
                        });
                    })
                    .body(|mut body| {
                        // For each loaded module we'll also need to display all of it's imports,
                        // they'll both be selectable values.
                        for module in &self.modules {
                            body.row(button_height, |mut row| {
                                // Check if it's selected or not
                                let module_enum = ModuleEnum::Title(to_rstr!(module.szModule));
                                let is_selected =
                                    Some(&module_enum) == self.selected_module.as_ref();

                                // First column will literally just be the module name but we'll
                                // need to make it a seletable ui widget
                                row.col(|ui| {
                                    let button = ui.add_sized(
                                        [ui.available_width(), 10.0],
                                        egui::SelectableLabel::new(
                                            is_selected,
                                            to_rstr!(module.szModule),
                                        ),
                                    );

                                    // Check if was clicked
                                    if button.clicked() {
                                        self.selected_module = Some(module_enum);
                                    }
                                });

                                if is_selected {
                                    row.col(|ui| {
                                        ui.label("bla bla bla");
                                    });
                                }
                            })
                        }
                        /*body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label("Hello");
                            });
                            row.col(|ui| {
                                ui.button("world!");
                            });
                        });*/
                    });
            });
    }

    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return "[>] Imports".to_string();
    }
}
