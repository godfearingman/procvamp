use super::AllocationEnum;
use super::{format_protection, format_state, format_type};
use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use windows::Win32::System::Memory::MEMORY_BASIC_INFORMATION;

/// Create out custom TabContent object for this specific tab, in this case it will be for our
/// allocation view
///
#[derive(Clone)]
pub struct AllocationView {
    pub selected_allocation_enum: Option<AllocationEnum>,
    pub selected_allocation: Option<MEMORY_BASIC_INFORMATION>,
    pub allocations: Vec<MEMORY_BASIC_INFORMATION>,
}

/// Form abstract link to TabContent
///
impl TabContent for AllocationView {
    /// Cursed ass code incoming not even gonna lie.
    fn ui(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                // Store static height because any subsequent call to ui.available_width()
                // in dynamic settings will just continously blow itself up with any
                // changes
                let total_width = ui.available_width();
                let panel_width = total_width;

                // On the left side panel we're going to display any allocations found which are
                // selectable, from there you can see more information on the other panel.
                egui::SidePanel::left("allocations_panel_imports_left")
                    .resizable(false)
                    .exact_width(panel_width)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // Create our table which will be used as a sort of listbox in this case,
                            // it's prettier.
                            TableBuilder::new(ui)
                                .column(Column::exact(panel_width * 0.2))
                                .column(Column::exact(panel_width * 0.2))
                                .column(Column::exact(panel_width * 0.2))
                                .column(Column::exact(panel_width * 0.2))
                                .column(Column::exact(panel_width * 0.15))
                                // Create our headers which will show what the value retrieved is showing
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.heading("Base Address");
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.heading("Region Size");
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.heading("Protection");
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.heading("State");
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.heading("Type");
                                        });
                                    });
                                })
                                // Iterate over every allocation and display the information
                                // in a pretty fashion, each allocation entry gets its own row
                                .body(|body| {
                                    let row_height = 30.0;
                                    let num_allocations = self.allocations.len();

                                    let mut selected_alloc_idx = None;

                                    body.rows(row_height, num_allocations, |mut row| {
                                        let row_index = row.index();

                                        if let Some(allocation) = self.allocations.get(row_index) {
                                            // Create enum member and check if this is selected
                                            let alloc_enum =
                                                AllocationEnum::Title(allocation.BaseAddress as _);
                                            let is_selected = Some(&alloc_enum)
                                                == self.selected_allocation_enum.as_ref();

                                            // Format the address, size & protection
                                            let base_address =
                                                format!("0x{:X}", allocation.BaseAddress as u64);
                                            let region_size =
                                                format!("0x{:X}", allocation.RegionSize as u32);

                                            // For the address column
                                            row.col(|ui| {
                                                ui.centered_and_justified(|ui| {
                                                    let label = ui.selectable_label(
                                                        is_selected,
                                                        base_address,
                                                    );
                                                    if label.clicked() {
                                                        selected_alloc_idx = Some(row_index);
                                                    }
                                                });
                                            });

                                            // For the size column
                                            row.col(|ui| {
                                                ui.centered_and_justified(|ui| {
                                                    let label = ui
                                                        .selectable_label(is_selected, region_size);
                                                    if label.clicked() {
                                                        selected_alloc_idx = Some(row_index);
                                                    }
                                                });
                                            });

                                            // For the protection column
                                            row.col(|ui| {
                                                ui.centered_and_justified(|ui| {
                                                    let label = ui.selectable_label(
                                                        is_selected,
                                                        format_protection(allocation.Protect.0),
                                                    );
                                                    if label.clicked() {
                                                        selected_alloc_idx = Some(row_index);
                                                    }
                                                });
                                            });

                                            // For the state column
                                            row.col(|ui| {
                                                ui.centered_and_justified(|ui| {
                                                    let label = ui.selectable_label(
                                                        is_selected,
                                                        format_state(allocation.State.0),
                                                    );
                                                    if label.clicked() {
                                                        selected_alloc_idx = Some(row_index);
                                                    }
                                                });
                                            });

                                            // For the type column
                                            row.col(|ui| {
                                                ui.centered_and_justified(|ui| {
                                                    let label = ui.selectable_label(
                                                        is_selected,
                                                        format_type(allocation.Type.0),
                                                    );
                                                    if label.clicked() {
                                                        selected_alloc_idx = Some(row_index);
                                                    }
                                                });
                                            });
                                        }
                                    });

                                    if let Some(idx) = selected_alloc_idx {
                                        if let Some(allocation) = self.allocations.get(idx) {
                                            let alloc_enum =
                                                AllocationEnum::Title(allocation.BaseAddress as _);
                                            self.selected_allocation_enum = Some(alloc_enum);
                                            self.selected_allocation = Some(*allocation);
                                        }
                                    }
                                });
                        });
                    });
            });
    }

    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return "[>] Allocations".to_string();
    }
}
