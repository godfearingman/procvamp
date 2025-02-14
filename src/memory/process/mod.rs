pub mod iterators;
pub mod process;
use thiserror::Error;

/// Define a macro for turning C strings into rustified strings, makes code cleaner on each call
///
#[macro_export]
macro_rules! to_rstr {
    ($arr:expr) => {
        String::from_utf8_lossy(
            $arr.iter()
                .szExeFile
                .iter()
                .map(|&c| c as u8)
                .take_while(|&c| c != 0)
                .collect::<Vec<u8>>(),
        )
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
}
