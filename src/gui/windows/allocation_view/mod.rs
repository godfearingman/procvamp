pub mod allocation_view;
use windows::Win32::System::Memory::*;

#[derive(PartialEq, Clone)]
pub enum AllocationEnum {
    Title(u64),
}

pub fn format_protection(protect_value: u32) -> String {
    let mut prot_abbr = String::new();

    // Read flag
    if (protect_value
        & (PAGE_READONLY.0 | PAGE_READWRITE.0 | PAGE_EXECUTE_READ.0 | PAGE_EXECUTE_READWRITE.0))
        != 0
    {
        prot_abbr.push('R');
    }

    // Write flag
    if (protect_value
        & (PAGE_READWRITE.0
            | PAGE_WRITECOPY.0
            | PAGE_EXECUTE_READWRITE.0
            | PAGE_EXECUTE_WRITECOPY.0))
        != 0
    {
        prot_abbr.push('W');
    }

    // Execute flag
    if (protect_value
        & (PAGE_EXECUTE.0
            | PAGE_EXECUTE_READ.0
            | PAGE_EXECUTE_READWRITE.0
            | PAGE_EXECUTE_WRITECOPY.0))
        != 0
    {
        prot_abbr.push('X');
    }

    // Special flags
    if (protect_value & PAGE_NOACCESS.0) != 0 {
        prot_abbr.push_str("NA");
    }
    if (protect_value & (PAGE_WRITECOPY.0 | PAGE_EXECUTE_WRITECOPY.0)) != 0 {
        prot_abbr.push_str("C");
    }
    if (protect_value & PAGE_GUARD.0) != 0 {
        prot_abbr.push_str("G");
    }
    if (protect_value & PAGE_NOCACHE.0) != 0 {
        prot_abbr.push_str("NC");
    }
    if (protect_value & PAGE_WRITECOMBINE.0) != 0 {
        prot_abbr.push_str("WC");
    }

    // If no flags were detected, mark as UNKNOWN
    if prot_abbr.is_empty() {
        String::from("UNK")
    } else {
        format!("{}", prot_abbr)
    }
}

pub fn format_state(state: u32) -> String {
    match state {
        s if s == MEM_COMMIT.0 => "COMMIT".to_string(),
        s if s == MEM_FREE.0 => "FREE".to_string(),
        s if s == MEM_RESERVE.0 => "RESERVE".to_string(),
        _ => format!("UNKNOWN (0x{:X})", state),
    }
}

pub fn format_type(mem_type: u32) -> String {
    match mem_type {
        t if t == MEM_IMAGE.0 => "IMAGE".to_string(),
        t if t == MEM_MAPPED.0 => "MAPPED".to_string(),
        t if t == MEM_PRIVATE.0 => "PRIVATE".to_string(),
        0 => "FREE".to_string(),
        _ => format!("UNKNOWN (0x{:X})", mem_type),
    }
}
