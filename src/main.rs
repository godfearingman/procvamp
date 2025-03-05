mod gui;
mod log;
mod memory;
use crate::gui::main::main::run_gui;
use crate::log::log::setup_logger;

use crate::memory::process::*;

fn main() -> eframe::Result<()> {
    if let Err(e) = setup_logger() {
        panic!("Failed to init logger with {e:?}");
    }
    ::log::debug!("Initialised logging system");

    run_gui()
}
