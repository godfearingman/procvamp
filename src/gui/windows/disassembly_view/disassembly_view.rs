use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use egui::{RichText, TextStyle, Ui};

// Create out custom TabContent object for this specific tab, in this case it will be for our
// disassembly view
//
#[derive(Clone)]
pub struct DisassemblyView {
    pub address_start: u64,
    //pub bytes: Vec<u8>,
}

// Form abstract link to TabContent
//
impl TabContent for DisassemblyView {
    // TODO: Disassemble the region from start to start + some_magic_value where the magic value is
    // according to the current resolution of the tab, we don't want to overread for no reason
    // UNLESS that is specifically set, I think we should read AT LEAST 512 bytes from anywhere to
    // prevent working set unless specified otherwise.
    //
    fn ui(&mut self, ui: &mut Ui) {
        // For now, we're just going to use some placeholder code to showcase how the output should
        // look like in the future once we get the disassembler actually set up
        //
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Eventually we'll need to switch to a method that figures out how much free
                    // space is currently available on the current tab (As it is resizable at the
                    // moment) and just display that amount. Or we can just read the default set
                    // size and since this is a scroll area, there isn't much bad that can happen.
                    //
                    for i in 0..512 {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 30.0;
                            ui.label(
                                RichText::new(format!("{:016X}", self.address_start + (i * 8)))
                                    .color(DARK_THEME.primary)
                                    .text_style(TextStyle::Monospace),
                            );
                            ui.label(
                                RichText::new("00 00 00 00 00 00 00 00")
                                    .color(DARK_THEME.secondary)
                                    .text_style(TextStyle::Monospace),
                            );
                            ui.label(
                                RichText::new("Disassembler text")
                                    .color(DARK_THEME.text_muted)
                                    .text_style(TextStyle::Monospace),
                            );
                        });
                    }
                });
            });
    }
    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return format!("[>] Disassembly ({:X})", self.address_start);
    }
}
