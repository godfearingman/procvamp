use windows::Win32::Foundation::{CloseHandle, ERROR_NO_MORE_FILES, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};

/// Define our custom iterator for modules, rust iterators are always better than duplicating code
/// and it will help simplify any future operations within process'
///
struct ModuleIterator {
    snap_handle: HANDLE,
    entry: MODULEENTRY32,
    is_first: bool,
}

impl ModuleIterator {
    /// Define our constructor to initialise snapshots and get ready to iterate through all entries
    ///
    pub unsafe fn new(pid: u32) -> anyhow::Result<Self> {
        // Get module snapshot
        let module_snapshot =
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE32 | TH32CS_SNAPMODULE, pid)?;
        // Setup module struct
        let mut module_entry = MODULEENTRY32::default();
        // Set struct size
        module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;

        Ok(Self {
            snap_handle: module_snapshot,
            entry: module_entry,
            is_first: true,
        })
    }
}

/// Define iterator type for struct
///
impl Iterator for ModuleIterator {
    /// Define our item, we'll only entry
    ///
    type Item = MODULEENTRY32;
    /// Define our .next() function, this will be where our iterator moves onto next entries when
    /// needed
    ///
    fn next(&mut self) -> Option<Self::Item> {
        // Check if this is the first entry
        if self.is_first {
            if Module32First(self.snap_handle, &mut self.entry).is_err() {
                return None;
            }
            self.is_first = false;
        } else {
            // Handle all other entries
            // Onto next entry
            match Module32Next(self.snap_handle, &mut self.entry) {
                Ok(_) => {}
                Err(_) => return None,
            }
        }
        // Clone as we're clearing it after this anyway
        let entry_clone = self.entry;
        // Clear buffer
        self.entry
            .szExeFile
            .iter_mut()
            .for_each(|e_byte| *e_byte = 0x0);

        Some(entry_clone)
    }
}

/// Define drop type for iterator
///
impl Drop for ModuleIterator {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.snap_handle).ok();
        }
    }
}
