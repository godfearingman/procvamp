use crate::gui::gui::TabContent;
use egui::Ui;

// Create out custom TabContent object for this specific tab, in this case it will be for our
// disassembly view
//
#[derive(Clone, Debug)]
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
        ui.label(".............");
    }
    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return format!("[>] Disassembly ({:X})", self.address_start);
    }
}
