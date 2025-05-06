use super::FunctionEnum;
use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use crate::pe::pe::{get_functions, get_pe_from_path, RuntimeFunction};
use egui::Ui;
use egui_extras::{Column, TableBuilder};
// Create out custom TabContent object for this specific tab, in this case it will be for our
// Function view
//
#[derive(Clone)]
pub struct FunctionView {
    pub process_path: Option<String>,
    pub fmap: Vec<RuntimeFunction>,
    pub selected_fn_ex: Option<RuntimeFunction>,
    pub selected_fn: Option<String>,
    pub selected_fn_enum: Option<FunctionEnum>,
    pub process_base: u64,
}

// Form abstract link to TabContent
//
impl TabContent for FunctionView {
    fn ui(&mut self, ui: &mut Ui) {
        // Populate functions map if it's empty
        //
        if self.fmap.is_empty() {
            let pe_file =
                self.process_path.as_ref().map(|path| path).and_then(
                    |path| match get_pe_from_path(path.clone()) {
                        Ok(pe) => Some(pe),
                        Err(_) => None,
                    },
                );
            if let Some(pe) = pe_file {
                self.fmap = get_functions(&pe).unwrap();
            }
        }

        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                egui::SidePanel::left("functions_panel")
                    .resizable(false)
                    .default_width(ui.available_width() * 0.5)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                            )
                            .show(ui, |ui| {
                                // Use TableBuilder for virtual scrolling
                                TableBuilder::new(ui)
                                    .column(Column::remainder())
                                    .body(|body| {
                                        let row_height = 20.0;
                                        let num_functions = self.fmap.len();

                                        // Track selected function outside the row closure
                                        let mut selected_fn_idx = None;

                                        body.rows(row_height, num_functions, |mut row| {
                                            let row_index = row.index();

                                            if let Some(func) = self.fmap.get(row_index) {
                                                let fn_name = format!(
                                                    "fn_{:x}",
                                                    self.process_base + func.begin_address as u64
                                                );

                                                // Create an enum member of all functions
                                                let fn_enum = FunctionEnum::Title(fn_name.clone());
                                                // Check if it was selected
                                                let is_selected = self.selected_fn_enum.as_ref()
                                                    == Some(&fn_enum);

                                                row.col(|ui| {
                                                    // The actual selectable button in question...
                                                    let button = ui.add_sized(
                                                        [ui.available_width(), 20.0],
                                                        egui::SelectableLabel::new(
                                                            is_selected,
                                                            &fn_name,
                                                        ),
                                                    );

                                                    // If clicked, track for selection after closure
                                                    if button.clicked() {
                                                        selected_fn_idx = Some(row_index);
                                                    }
                                                });
                                            }
                                        });

                                        // Process selection outside the row closure
                                        if let Some(idx) = selected_fn_idx {
                                            if let Some(func) = self.fmap.get(idx) {
                                                let fn_name = format!(
                                                    "fn_{:x}",
                                                    self.process_base + func.begin_address as u64
                                                );

                                                // If it was clicked then store it
                                                let fn_enum = FunctionEnum::Title(fn_name.clone());
                                                self.selected_fn_enum = Some(fn_enum);
                                                self.selected_fn = Some(fn_name);
                                                self.selected_fn_ex = Some(func.clone());
                                            }
                                        }
                                    });
                            });
                    }); // Create the right panel now which will show information about the currently
                        // selected module
                egui::CentralPanel::default()
                    .frame(egui::Frame::none().inner_margin(egui::Margin {
                        left: ui.available_width() * 0.1, // Add left margin for spacing
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    }))
                    .show_inside(ui, |ui| {
                        // Create a list of all modules, once clicked it will store the selected
                        // module for the right panel to display information about it
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                if let Some(func) = &self.selected_fn_ex {
                                    ui.label(format!("Begin : {:X}", func.begin_address));
                                    ui.label(format!("End : {:X}", func.end_address));
                                    ui.label(format!("Unwind : {:X}", func.unwind_info));
                                }
                            });
                        });
                    });
            });
    }
    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return String::from("[>] Function view");
    }
}
