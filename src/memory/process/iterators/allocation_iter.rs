use super::super::process::Process;
use std::ffi::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Memory::{
    VirtualQueryEx, MEMORY_BASIC_INFORMATION, PAGE_PROTECTION_FLAGS,
};

/// Create an allocation iteration struct, this will let us iterate through every valid allocation
/// within a process that's passed to the 'new' constructor
///
pub struct Allocation {
    process_handle: HANDLE,
    entry: MEMORY_BASIC_INFORMATION,
    curr_addr: u64,
}

impl Allocation {
    /// Simple constructor for our iteration, we'll need the handle and base address to start from
    ///
    pub unsafe fn new(process: &mut Process) -> anyhow::Result<Self> {
        let handle = process.get_handle()?;

        Ok(Self {
            process_handle: handle,
            entry: MEMORY_BASIC_INFORMATION::default(),
            curr_addr: 0,
        })
    }
    /// Filter allocations with a certain protection
    ///
    pub fn with_protection(
        self,
        prot: PAGE_PROTECTION_FLAGS,
    ) -> impl Iterator<Item = MEMORY_BASIC_INFORMATION> {
        self.filter(move |entry| entry.Protect == prot)
    }
}

/// Implement the Iterator type for our struct
///
impl Iterator for Allocation {
    /// Our type will just be a MEMORY_BASIC_INFORMATION
    ///
    type Item = MEMORY_BASIC_INFORMATION;
    /// Implement the next function, this will effectively just get the last allocation and
    /// virtualquery onto the next allocation after curr.regionsize
    ///
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            // Start at base address and move on, updating curr_addr each iteration and checking if
            // virtualquery returned 0

            let call_result = VirtualQueryEx(
                self.process_handle,
                Some(self.curr_addr as *const c_void),
                &mut self.entry,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );

            if call_result == 0 {
                return None;
            }

            self.curr_addr = match self.curr_addr.checked_add(self.entry.RegionSize as u64) {
                Some(addr) => addr,
                None => return None,
            };

            Some(self.entry)
        }
    }
}
