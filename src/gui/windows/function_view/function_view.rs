use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use egui::{RichText, TextStyle, Ui};
use std::collections::HashMap;

// Create out custom TabContent object for this specific tab, in this case it will be for our
// disassembly view
//
#[derive(Clone)]
pub struct FunctionView {
    // Our hashmap where all functions will be stored
    //
    fmap: HashMap<String, u64>,
    // Our currently selected function
    //
    selected_fn: Option<(String, u64)>,
}

// Since we're using a map, we'll need to implement a constructor as this will only ever be
// constructed once and used from then on, we'll also implement some functions to retrieve and
// store
//
impl FunctionView {
    pub fn new() -> Self {
        Self {
            fmap: HashMap::new(),
            selected_fn: None,
        }
    }
    // Basic getter
    //
    pub fn get(&self, k: String) -> Option<&u64> {
        Some(self.fmap.get(&k))?
    }
    // Basic setter
    //
    pub fn set(&mut self, k: String, v: u64) -> Option<u64> {
        self.fmap.insert(k, v)
    }
}

// Form abstract link to TabContent
//
impl TabContent for FunctionView {
    fn ui(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Loop through every single entry in the hashmap and display them here in its
                    // own tab
                    //
                    egui::Frame::none()
                        .fill(DARK_THEME.background_dark)
                        .inner_margin(1.0)
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for (name, addr) in &self.fmap {
                                    ui.spacing_mut().item_spacing.x = 5.0;
                                    // Check if it's currently the selected name or not
                                    //
                                    let is_fn_selected = self
                                        .selected_fn
                                        .as_ref()
                                        .map_or(false, |(selected_name, _)| selected_name == name);

                                    ui.horizontal(|ui| {
                                        // Display as an option
                                        //
                                        let text =
                                            RichText::new(name).text_style(TextStyle::Monospace);

                                        // Check if selected for different colour.
                                        //
                                        let text = text.color(if is_fn_selected {
                                            DARK_THEME.primary
                                        } else {
                                            DARK_THEME.highlight
                                        });

                                        // Display as a selectable element
                                        //
                                        if ui.selectable_label(is_fn_selected, text).clicked() {
                                            self.selected_fn = Some((name.to_string(), *addr));
                                            // TODO: Have some buttons later on for a right click that
                                            // will send to disassembler view or any other main view.
                                            //
                                        }

                                        // Display address next to it
                                        //
                                        ui.label(
                                            RichText::new(format!("{:016X}", addr))
                                                .color(DARK_THEME.secondary)
                                                .text_style(TextStyle::Monospace),
                                        );
                                    });
                                }
                            });
                        })
                });
            });
    }
    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return String::from("[>] Function view");
    }
}
