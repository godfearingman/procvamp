pub mod iterators;
pub mod process;
use thiserror::Error;

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
}
