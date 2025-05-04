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
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
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

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Display all decoded instructions
                    for (_i, (addr, instruction, formatted)) in self.instructions.iter().enumerate()
                    {
                        ui.horizontal(|ui| {
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

                            ui.label(
                                RichText::new(bytes_str)
                                    .color(DARK_THEME.secondary)
                                    .text_style(TextStyle::Monospace),
                            );

                            // Display disassembled text
                            ui.label(
                                RichText::new(formatted)
                                    .color(DARK_THEME.text_muted)
                                    .text_style(TextStyle::Monospace),
                            );
                        });
                    }
                });
            });
    }

    /// Handle our name of the tab
    ///
    fn title(&self) -> String {
        return format!("[>] Disassembly ({:X})", self.address_start);
    }
}
