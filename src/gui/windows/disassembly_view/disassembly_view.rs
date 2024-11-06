use crate::gui::gui::TabContent;
use egui::{Color32, RichText, TextStyle, Ui};

// Create out custom TabContent object for this specific tab, in this case it will be for our
// disassembly view
//
#[derive(Clone, Debug)]
pub struct DisassemblyView {
    pub address_start: u64,
    //pub bytes: Vec<u8>,
}

// Define our vampire-themed colors
const THEME_COLORS: ThemeColors = ThemeColors {
    // Deep blood red
    primary: Color32::from_rgb(140, 0, 0),
    // Rich purple
    secondary: Color32::from_rgb(83, 53, 74),
    // Darker blood red for backgrounds
    background_dark: Color32::from_rgb(20, 0, 0),
    // Desaturated blood red for text
    text_muted: Color32::from_rgb(171, 103, 103),
    // Brighter blood red for highlights
    highlight: Color32::from_rgb(196, 27, 27),
};

struct ThemeColors {
    primary: Color32,
    secondary: Color32,
    background_dark: Color32,
    text_muted: Color32,
    highlight: Color32,
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
            .fill(THEME_COLORS.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for i in 0..20 {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 30.0;
                            ui.label(
                                RichText::new(format!("{:X}", self.address_start + (i * 8)))
                                    .color(THEME_COLORS.primary)
                                    .text_style(TextStyle::Monospace),
                            );
                            ui.label(
                                RichText::new("00 00 00 00 00 00 00 00")
                                    .color(THEME_COLORS.secondary)
                                    .text_style(TextStyle::Monospace),
                            );
                            ui.label(
                                RichText::new("Disassembler text")
                                    .color(THEME_COLORS.text_muted)
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
