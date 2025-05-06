use super::{format_bytes_to_string, InstructionFormatterOutput};
use crate::gui::gui::selectable_bp;
use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use crate::process::Process;
use egui::{RichText, TextStyle, Ui};
use iced_x86::IntelFormatter;
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction};

/// Create out custom TabContent object for this specific tab, in this case it will be for our
/// Disassembly view
///
#[derive(Clone)]
pub struct DisassemblyView {
    pub address_start: u64,
    pub process: Process,
    pub bytes: Vec<u8>,
    pub instructions: Vec<(u64, Instruction, String)>,
    pub bytes_read: usize,
}

impl DisassemblyView {
    /// Helper function to manually refresh the disassembly
    ///
    fn refresh_disassembly(&mut self) -> anyhow::Result<()> {
        const READ_SIZE: usize = 512;

        unsafe {
            self.bytes = self
                .process
                .read_bytes_paged(self.address_start as usize, READ_SIZE)?;
            self.bytes_read = self.bytes.len();

            self.disassemble_bytes();
        }

        Ok(())
    }

    fn parse_bytes_from_string(&self, input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
        input
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16))
            .collect()
    }

    fn patch_bytes(&mut self, addr: u64, new_bytes: &[u8]) -> anyhow::Result<()> {
        // First, write the bytes to the process
        unsafe {
            self.process.write_bytes(addr as usize, new_bytes)?;

            // Then refresh our view to show the updated bytes
            self.refresh_disassembly()?;
        }

        Ok(())
    }

    /// Helper function to  quickly disassemble the bytes that have been read
    ///
    fn disassemble_bytes(&mut self) {
        self.instructions.clear();

        if self.bytes.is_empty() {
            return;
        }

        // Create a decoder for 64-bit code
        let mut decoder = Decoder::with_ip(
            64,                   // 64-bit code
            &self.bytes,          // Code buffer
            self.address_start,   // IP (instruction pointer)
            DecoderOptions::NONE, // No special options
        );

        // Format the decoded instructions
        let mut formatter = IntelFormatter::new();
        formatter.options_mut().set_first_operand_char_index(10);

        // Decode all instructions
        let mut instruction = Instruction::default();
        let mut output = InstructionFormatterOutput::new();

        while decoder.can_decode() {
            let offset = decoder.position();
            decoder.decode_out(&mut instruction);

            // Format the instruction to a string
            output.clear();
            formatter.format(&instruction, &mut output);
            let formatted = output.to_string();

            // Store the instruction with its address and formatted text
            self.instructions.push((
                self.address_start + offset as u64,
                instruction.clone(),
                formatted,
            ));
        }
    }
}

/// Form abstract link to TabContent
///
impl TabContent for DisassemblyView {
    fn ui(&mut self, ui: &mut Ui) {
        // This is extremely ugly but we barely have a choice here because of how egui works, we'll
        // be annoyed by the borrow checker if we try to make these struct members and attempt to
        // modify later on
        static mut SELECTED_ADDR: Option<u64> = None;
        static mut BYTES_TO_PATCH: String = String::new();
        static mut PATCH_REQUESTED: bool = false;
        static mut PATCH_DATA: Option<(u64, Vec<u8>)> = None;

        let selected_addr = unsafe { &mut SELECTED_ADDR };
        let bytes_to_patch = unsafe { &mut BYTES_TO_PATCH };
        let patch_requested = unsafe { &mut PATCH_REQUESTED };
        let patch_data = unsafe { &mut PATCH_DATA };

        // Check if we need to apply a patch from the previous frame
        if *patch_requested {
            *patch_requested = false;

            if let Some((addr, bytes)) = patch_data.take() {
                let _ = self.patch_bytes(addr, &bytes);
                *selected_addr = None;
                *bytes_to_patch = String::new();
            }
        }

        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                // Existing top bar UI
                ui.horizontal(|ui| {
                    if ui.button("Refresh").clicked() {
                        let _ = self.refresh_disassembly();
                    }

                    ui.separator();

                    ui.label("Go to address:");

                    static mut ADDRESS_INPUT: String = String::new();
                    let addr_input = unsafe { &mut ADDRESS_INPUT };

                    if ui.text_edit_singleline(addr_input).lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        if let Ok(addr) =
                            u64::from_str_radix(addr_input.trim_start_matches("0x"), 16)
                        {
                            self.address_start = addr;
                            let _ = self.refresh_disassembly();
                        }
                    }
                });

                ui.separator();

                // Add status label for patching
                if selected_addr.is_some() {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(
                                "Editing bytes - press Enter to apply or Escape to cancel",
                            )
                            .color(DARK_THEME.primary),
                        );
                    });
                    ui.separator();
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Display all decoded instructions
                    for (_i, (addr, instruction, formatted)) in self.instructions.iter().enumerate()
                    {
                        // We're going to check if the current line is selected or not, if it is
                        // we'll add the editing functionality.
                        let is_selected = selected_addr.map_or(false, |sel_addr| sel_addr == *addr);

                        let row = ui.horizontal(|ui| {
                            // Spacing for the breakpoint button
                            ui.spacing_mut().item_spacing.x = 5.0;
                            selectable_bp(ui, None);

                            ui.spacing_mut().item_spacing.x = 30.0;

                            // Display address
                            ui.label(
                                RichText::new(format!("{:016X}", addr))
                                    .color(DARK_THEME.primary)
                                    .text_style(TextStyle::Monospace),
                            );

                            // Get bytes for this instruction
                            let instr_size = instruction.len();
                            let offset = (*addr - self.address_start) as usize;

                            let bytes_str = if offset + instr_size <= self.bytes.len() {
                                format_bytes_to_string(&self.bytes[offset..offset + instr_size])
                            } else {
                                "?? ?? ?? ??".to_string()
                            };

                            // If it is selected, we're just going to use the basic text edit
                            // functionality for the label, otherwise we'll use a regular clickable
                            // label.
                            if is_selected {
                                let response = ui.text_edit_singleline(bytes_to_patch);

                                // Handle keypresses for exiting or entering
                                if response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    // Parse the new bytes
                                    if let Ok(new_bytes) =
                                        self.parse_bytes_from_string(bytes_to_patch)
                                    {
                                        // On next frame, it'll replace the bytes, this is because
                                        // we have an immutable object of self here so we cannot
                                        // directly update it here.
                                        *patch_data = Some((*addr, new_bytes));
                                        *patch_requested = true;
                                    }
                                }
                                // Cancelling...
                                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                    *selected_addr = None;
                                    *bytes_to_patch = String::new();
                                }
                            } else {
                                // Show regular label
                                if ui
                                    .add(egui::Label::new(
                                        RichText::new(bytes_str.clone())
                                            .color(DARK_THEME.secondary)
                                            .text_style(TextStyle::Monospace),
                                    ))
                                    .double_clicked()
                                {
                                    // If double clicked, we set this one to be the selected object
                                    *selected_addr = Some(*addr);
                                    *bytes_to_patch = bytes_str.trim().to_string();
                                }
                            }

                            ui.label(
                                RichText::new(formatted)
                                    .color(DARK_THEME.text_muted)
                                    .text_style(TextStyle::Monospace),
                            );
                        });

                        if is_selected {
                            row.response.highlight();
                        }
                    }
                });
            });
    }

    fn title(&self) -> String {
        return format!("[>] Disassembly ({:X})", self.address_start);
    }
}
