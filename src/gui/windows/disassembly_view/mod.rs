pub mod disassembly_view;
use iced_x86::{FormatterOutput, FormatterTextKind};

/// Our custom formatter for our disassembler
///
struct InstructionFormatterOutput {
    text: String,
}

impl InstructionFormatterOutput {
    fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    fn clear(&mut self) {
        self.text.clear();
    }

    fn to_string(&self) -> String {
        self.text.clone()
    }
}

impl FormatterOutput for InstructionFormatterOutput {
    fn write(&mut self, text: &str, _kind: FormatterTextKind) {
        self.text.push_str(text);
    }
}

/// Helper function to format bytes as a hex string
///
fn format_bytes_to_string(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "??".to_string();
    }

    let mut byte_str = String::new();
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 {
            byte_str.push(' ');
        }
        byte_str.push_str(&format!("{:02X}", byte));
    }

    // Pad to a consistent length for alignment
    while byte_str.len() < 23 {
        byte_str.push(' ');
    }

    byte_str
}
