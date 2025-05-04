use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use crate::process::Process;
use egui::Ui;
use egui_extras::{Column, TableBuilder};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum ScanType {
    #[default]
    Exact,
    BiggerThan,
    SmallerThan,
}

// NOTE: could add a sub enum for int or float variants.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum ValueType {
    Byte,
    TwoBytes,
    #[default]
    FourBytes,
    EightBytes,
}

#[derive(Clone, Default, Debug)]
pub struct ScanResult {
    pub address: u64,
    pub value: String,
    // could add other stuff here, like previous and first.
}

impl ScanResult {
    pub fn new(address: u64, value: String) -> Self {
        Self { address, value }
    }
}

/// Create out custom TabContent object for this specific tab, in this case it will be for our
/// Scanner view
///
#[derive(Clone)]
pub struct ScannerView {
    pub process: Process,
    /// Is search value a hex number.
    pub is_hex: bool,
    pub scan_type: ScanType,
    pub value_type: ValueType,
    /// Used to determine if it uses alignment or not.
    pub fast_scan: bool,
    pub value: String,
    // NOTE: could also do some pubsub type deal.
    pub results: Vec<ScanResult>,
}

/// Form abstract link to TabContent
///
impl TabContent for ScannerView {
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
                let panel_width = total_width / 2.7;
                // Left panel is going for be for actual found values, you can double click and it
                // will be sent to scanner results.
                egui::SidePanel::left("scanner_panel_left")
                    .resizable(false)
                    .exact_width(panel_width)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("scanner_scroll")
                            .show(ui, |ui| {
                                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                    TableBuilder::new(ui)
                                        .column(Column::exact(120.0).clip(false))
                                        .column(Column::remainder().clip(true))
                                        .auto_shrink([false; 2])
                                        //.striped(true)
                                        .header(20.0, |mut header| {
                                            header.col(|ui| {
                                                ui.heading("Address");
                                            });
                                            header.col(|ui| {
                                                ui.heading("Value");
                                            });
                                        })
                                        .body(|body| {
                                            // Since we're dealing with possibly a large amount of
                                            // results, we're going to be using the virtual
                                            // scrolling functionality from the body.rows function
                                            let row_height = 30.0;
                                            body.rows(row_height, self.results.len(), |mut row| {
                                                let row_index = row.index();
                                                if let Some(result) = self.results.get(row_index) {
                                                    row.col(|ui| {
                                                        ui.label(format!("0x{:X}", result.address));
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(format!("{}", result.value));
                                                    });
                                                }
                                            });
                                        });
                                });
                            });
                    });
                // The last frame is going to just be for all the scan options and inputs etc..
                egui::CentralPanel::default()
                    .frame(egui::Frame::none().inner_margin(egui::Margin {
                        left: ui.available_width() * 0.1,
                        right: ui.available_width() * 0.1,
                        top: 3.0,
                        bottom: 0.0,
                    }))
                    .show_inside(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            // show the value text box and checkbox right next to it.
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                                ui.text_edit_singleline(&mut self.value);
                                ui.checkbox(&mut self.is_hex, "Hex");
                            });

                            ui.add_space(7.0);

                            // combo box for the user to decide the type of the scan.
                            egui::ComboBox::from_label("Scan Type")
                                .selected_text(format!("{:?}", self.scan_type))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.scan_type,
                                        ScanType::Exact,
                                        "Exact",
                                    );
                                    ui.selectable_value(
                                        &mut self.scan_type,
                                        ScanType::BiggerThan,
                                        "Bigger Than",
                                    );
                                    ui.selectable_value(
                                        &mut self.scan_type,
                                        ScanType::SmallerThan,
                                        "Smaller Than",
                                    );
                                });

                            ui.add_space(3.0);

                            // combo box for the user to decide the type of the value alignment.
                            egui::ComboBox::from_label("Value Type")
                                .selected_text(format!("{:?}", self.value_type))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.value_type,
                                        ValueType::Byte,
                                        "Byte",
                                    );
                                    ui.selectable_value(
                                        &mut self.value_type,
                                        ValueType::TwoBytes,
                                        "Two Bytes",
                                    );
                                    ui.selectable_value(
                                        &mut self.value_type,
                                        ValueType::FourBytes,
                                        "Four Bytes",
                                    );
                                    ui.selectable_value(
                                        &mut self.value_type,
                                        ValueType::EightBytes,
                                        "Eight Bytes",
                                    );
                                });

                            ui.add_space(3.0);
                            ui.checkbox(&mut self.fast_scan, "Fast");
                            ui.add_space(7.0);

                            let scan_button = ui.button("Scan");
                            if scan_button.clicked() {
                                self.perform_scan()
                            }
                        });
                    });
            });
    }

    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return "[>] Scanner".to_string();
    }
}

impl ScannerView {
    /// Performs a scan based on the current settings
    fn perform_scan(&mut self) {
        // Parse the input value based on whether it's hex or decimal
        let parse_result = if self.is_hex {
            // Remove '0x' prefix if present
            let value_str = self.value.trim_start_matches("0x");
            match self.value_type {
                ValueType::Byte => u8::from_str_radix(value_str, 16).map(|v| v as u64),
                ValueType::TwoBytes => u16::from_str_radix(value_str, 16).map(|v| v as u64),
                ValueType::FourBytes => u32::from_str_radix(value_str, 16).map(|v| v as u64),
                ValueType::EightBytes => u64::from_str_radix(value_str, 16),
            }
        } else {
            self.value.parse::<u64>()
        };

        let value = match parse_result {
            Ok(v) => v,
            Err(_) => {
                // Handle parse error - maybe show in UI
                return;
            }
        };

        // Perform the scan based on value type
        let results = match self.value_type {
            ValueType::Byte => unsafe {
                self.process.find_data(
                    self.scan_type.clone(),
                    self.value_type.clone(),
                    value as u8,
                    self.fast_scan,
                )
            },
            ValueType::TwoBytes => unsafe {
                self.process.find_data(
                    self.scan_type.clone(),
                    self.value_type.clone(),
                    value as u16,
                    self.fast_scan,
                )
            },
            ValueType::FourBytes => unsafe {
                self.process.find_data(
                    self.scan_type.clone(),
                    self.value_type.clone(),
                    value as u32,
                    self.fast_scan,
                )
            },
            ValueType::EightBytes => unsafe {
                self.process.find_data(
                    self.scan_type.clone(),
                    self.value_type.clone(),
                    value,
                    self.fast_scan,
                )
            },
        };

        // Update results
        match results {
            Ok(scan_results) => {
                self.results = scan_results;
            }
            Err(e) => {
                // Handle error - maybe show in UI
                eprintln!("Scan failed: {}", e);
            }
        }
    }
}
