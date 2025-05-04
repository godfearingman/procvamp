pub mod iterators;
pub mod process;
use crate::gui::windows::scanner_view::scanner_view::ScanResult;
use crate::gui::windows::scanner_view::scanner_view::ScanType;
use crate::gui::windows::scanner_view::scanner_view::ValueType;
use thiserror::Error;

/// Define a macro for turning C strings into rustified strings, makes code cleaner on each call
///
#[macro_export]
macro_rules! to_rstr {
    ($arr:expr) => {
        String::from_utf8(
            $arr.iter()
                .map(|&c| c as u8)
                .take_while(|&c| c != 0)
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    };
}

/// Define our custom errors so that we can catch these specifically later and prevent a panick and
/// instead have some sort of messagebox error handler
///
#[derive(Error, Debug)]
pub enum ProcessErrors {
    #[error("Process fields are invalid, Process::find() must be called initially")]
    InvalidProcessFields,
    #[error("Handle is already invalid, Process::get_handle() failed or not called")]
    InvalidHandle,
    #[error("Failed to find process '{process_name}")]
    ProcessNotFound { process_name: String },
    #[error("Failed to find module '{module_name}")]
    ModuleNotFound { module_name: String },
}

/// Helper function to simply convert value to bytes for our funciton to take in
///
fn convert_value_to_bytes<T>(value: &T, value_type: &ValueType) -> anyhow::Result<Vec<u8>>
where
    T: 'static,
{
    let value_any: &dyn std::any::Any = value;
    match value_type {
        ValueType::Byte => {
            if let Some(v) = value_any.downcast_ref::<u8>() {
                Ok(vec![*v])
            } else {
                Err(anyhow::anyhow!("Expected u8 for Byte type"))
            }
        }
        ValueType::TwoBytes => {
            if let Some(v) = value_any.downcast_ref::<u16>() {
                Ok(v.to_le_bytes().to_vec())
            } else {
                Err(anyhow::anyhow!("Expected u16 for TwoBytes type"))
            }
        }
        ValueType::FourBytes => {
            if let Some(v) = value_any.downcast_ref::<u32>() {
                Ok(v.to_le_bytes().to_vec())
            } else {
                Err(anyhow::anyhow!("Expected u32 for FourBytes type"))
            }
        }
        ValueType::EightBytes => {
            if let Some(v) = value_any.downcast_ref::<u64>() {
                Ok(v.to_le_bytes().to_vec())
            } else {
                Err(anyhow::anyhow!("Expected u64 for EightBytes type"))
            }
        }
    }
}

/// Helper function to compare values based on scan type used within our scanning function
///
fn compare_values(
    memory_bytes: &[u8],
    value_bytes: &[u8],
    scan_type: &ScanType,
    value_type: &ValueType,
) -> anyhow::Result<bool> {
    match (scan_type, value_type) {
        (ScanType::Exact, _) => Ok(memory_bytes == value_bytes),
        (ScanType::BiggerThan, ValueType::Byte) => {
            let memory_val = memory_bytes[0];
            let target_val = value_bytes[0];
            Ok(memory_val > target_val)
        }
        (ScanType::BiggerThan, ValueType::TwoBytes) => {
            let memory_val = u16::from_le_bytes([memory_bytes[0], memory_bytes[1]]);
            let target_val = u16::from_le_bytes([value_bytes[0], value_bytes[1]]);
            Ok(memory_val > target_val)
        }
        (ScanType::BiggerThan, ValueType::FourBytes) => {
            let memory_val = u32::from_le_bytes([
                memory_bytes[0],
                memory_bytes[1],
                memory_bytes[2],
                memory_bytes[3],
            ]);
            let target_val = u32::from_le_bytes([
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
            ]);
            Ok(memory_val > target_val)
        }
        (ScanType::BiggerThan, ValueType::EightBytes) => {
            let memory_val = u64::from_le_bytes([
                memory_bytes[0],
                memory_bytes[1],
                memory_bytes[2],
                memory_bytes[3],
                memory_bytes[4],
                memory_bytes[5],
                memory_bytes[6],
                memory_bytes[7],
            ]);
            let target_val = u64::from_le_bytes([
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
                value_bytes[4],
                value_bytes[5],
                value_bytes[6],
                value_bytes[7],
            ]);
            Ok(memory_val > target_val)
        }
        (ScanType::SmallerThan, ValueType::Byte) => {
            let memory_val = memory_bytes[0];
            let target_val = value_bytes[0];
            Ok(memory_val < target_val)
        }
        (ScanType::SmallerThan, ValueType::TwoBytes) => {
            let memory_val = u16::from_le_bytes([memory_bytes[0], memory_bytes[1]]);
            let target_val = u16::from_le_bytes([value_bytes[0], value_bytes[1]]);
            Ok(memory_val < target_val)
        }
        (ScanType::SmallerThan, ValueType::FourBytes) => {
            let memory_val = u32::from_le_bytes([
                memory_bytes[0],
                memory_bytes[1],
                memory_bytes[2],
                memory_bytes[3],
            ]);
            let target_val = u32::from_le_bytes([
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
            ]);
            Ok(memory_val < target_val)
        }
        (ScanType::SmallerThan, ValueType::EightBytes) => {
            let memory_val = u64::from_le_bytes([
                memory_bytes[0],
                memory_bytes[1],
                memory_bytes[2],
                memory_bytes[3],
                memory_bytes[4],
                memory_bytes[5],
                memory_bytes[6],
                memory_bytes[7],
            ]);
            let target_val = u64::from_le_bytes([
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
                value_bytes[4],
                value_bytes[5],
                value_bytes[6],
                value_bytes[7],
            ]);
            Ok(memory_val < target_val)
        }
    }
}

/// Last helper function to extract value as string and send it back to return for our scanning
/// function
///
fn extract_value(memory_bytes: &[u8], value_type: &ValueType) -> String {
    match value_type {
        ValueType::Byte => {
            format!("{}", memory_bytes[0])
        }
        ValueType::TwoBytes => {
            let val = u16::from_le_bytes([memory_bytes[0], memory_bytes[1]]);
            format!("{}", val)
        }
        ValueType::FourBytes => {
            let val = u32::from_le_bytes([
                memory_bytes[0],
                memory_bytes[1],
                memory_bytes[2],
                memory_bytes[3],
            ]);
            format!("{}", val)
        }
        ValueType::EightBytes => {
            let val = u64::from_le_bytes([
                memory_bytes[0],
                memory_bytes[1],
                memory_bytes[2],
                memory_bytes[3],
                memory_bytes[4],
                memory_bytes[5],
                memory_bytes[6],
                memory_bytes[7],
            ]);
            format!("{}", val)
        }
    }
}
