use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};

/// Define our custom iterator for proesses, rust iterators are always better than duplicating code
/// and it will help simplify any future operations within process'
///
pub struct ProcessIterator {
    snap_handle: HANDLE,
    entry: PROCESSENTRY32,
    is_first: bool,
}

impl ProcessIterator {
    /// Define our constructor to initialise snapshots and get ready to iterate through all entries
    ///
    pub unsafe fn new() -> anyhow::Result<Self> {
        // Create a process snapshot
        let proc_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        // Setup process entry
        let mut proc_entry = PROCESSENTRY32::default();
        // Set struct size
        proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        Ok(Self {
            snap_handle: proc_snapshot,
            entry: proc_entry,
            is_first: true,
        })
    }
}

/// Define iterator type for struct
///
impl Iterator for ProcessIterator {
    /// Define our item, we'll only need name + pid
    ///
    type Item = (String, u32);
    /// Define our .next() function, this will be where our iterator moves onto next entries when
    /// needed
    ///
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            // Check if this is the first entry
            if self.is_first {
                if Process32First(self.snap_handle, &mut self.entry).is_err() {
                    return None;
                }
                self.is_first = false;
            } else {
                // Handle all other entries
                // Onto next entry
                match Process32Next(self.snap_handle, &mut self.entry) {
                    Ok(_) => {}
                    Err(_) => return None,
                }
            }
            // Get process name
            let proc_name: String = String::from_utf8(
                self.entry
                    .szExeFile
                    .iter()
                    .map(|&c| c as u8)
                    .take_while(|&c| c != 0)
                    .collect::<Vec<u8>>(),
            )
            .unwrap();
            // Clear buffer
            self.entry
                .szExeFile
                .iter_mut()
                .for_each(|e_byte| *e_byte = 0x0);

            Some((proc_name, self.entry.th32ProcessID))
        }
    }
}

/// Define drop type for iterator
///
impl Drop for ProcessIterator {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.snap_handle).ok();
        }
    }
}
