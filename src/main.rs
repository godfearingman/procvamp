mod gui;
mod memory;
use crate::gui::main::main::run_gui;

fn main() -> eframe::Result<()> {
    run_gui()
}
