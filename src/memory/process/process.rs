use std::ffi::c_void;
use windows::Win32::Foundation::{CloseHandle, ERROR_NO_MORE_FILES, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

#[derive(Debug, Clone)]
pub struct Process {
    process_handle: HANDLE,
    process_id: u32,
}

impl Process {
    /// Constructor to find a process by name and extract all the key information we'll need to perform analysis on said process
    pub unsafe fn find(name_of_process: &str) -> anyhow::Result<Self> {
        // Create a process snapshot
        let proc_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        // Setup process entry
        let mut proc_entry = PROCESSENTRY32::default();
        // Set struct size
        proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        // Check first process
        Process32First(proc_snapshot, &mut proc_entry)?;
        // Loop through all processes
        loop {
            // Get process name
            let proc_name: String = String::from_utf8_lossy(&proc_entry.szExeFile[..])
                .trim_end_matches('\0')
                .to_string();
            // Check name
            if &proc_name == name_of_process {
                CloseHandle(proc_snapshot)?;
                return Ok(Self {
                    // Set Process ID
                    process_id: proc_entry.th32ProcessID,
                    // Open a handle and set it to our field
                    process_handle: OpenProcess(
                        PROCESS_ALL_ACCESS,
                        false,
                        proc_entry.th32ProcessID,
                    )
                    .unwrap(),
                });
            }
            // Clear buffer
            proc_entry
                .szExeFile
                .iter_mut()
                .for_each(|e_byte| *e_byte = 0x0);
            // Onto next entry
            match Process32Next(proc_snapshot, &mut proc_entry) {
                Ok(_) => continue,
                Err(e) if e.raw_os_error() == Some(ERROR_NO_MORE_FILES as _) => break,
                Err(e) => {
                    CloseHandle(proc_snapshot)?;
                    return Err(e.into());
                }
            }
        }
        CloseHandle(proc_snapshot)?;
        return Ok(Self {
            // Set Process ID
            process_id: u32::default(),
            // Open a handle and set it to our field
            process_handle: HANDLE::default(),
        });
    }
    /// Write value of type T to the given process at location addr_to_write
    pub unsafe fn write<T>(&self, addr_to_write: usize, value_to_write: T) -> anyhow::Result<()> {
        WriteProcessMemory(
            self.process_handle,
            addr_to_write as *const c_void,
            &value_to_write as *const T as *const c_void,
            std::mem::size_of::<T>(),
            None,
        );
        Ok(())
    }
    /// Write an array of bytes to the given process at location addr_to_write
    pub unsafe fn write_bytes(
        &self,
        addr_to_write: usize,
        value_to_write: &[u8],
    ) -> anyhow::Result<()> {
        let val_ptr = value_to_write.as_ptr() as *const c_void;
        WriteProcessMemory(
            self.process_handle,
            addr_to_write as *const c_void,
            val_ptr,
            std::mem::size_of_val(value_to_write),
            None,
        );
        Ok(())
    }
    /// Read memory of type T from the process at the given location addr_to_read
    pub unsafe fn read<T /*: std::fmt::Display*/>(&self, addr_to_read: usize) -> anyhow::Result<T> {
        let mut buffer_vec: Vec<u8> = vec![0; std::mem::size_of::<T>()];
        ReadProcessMemory(
            self.process_handle,
            addr_to_read as *const c_void,
            buffer_vec.as_mut_ptr() as _,
            std::mem::size_of::<T>(),
            None,
        )?;
        // Create an uninitialized value of type T
        let mut result_value: std::mem::MaybeUninit<T> = std::mem::MaybeUninit::uninit();
        // Use copy_nonoverlapping to copy the bytes from the buffer vector to the target type
        std::ptr::copy_nonoverlapping(
            buffer_vec.as_ptr(),
            result_value.as_mut_ptr() as *mut u8,
            std::mem::size_of::<T>(),
        );
        // Convert from MaybeUninit<T> to T
        Ok(result_value.assume_init())
    }
    /// Read bytes from the process at the given location addr_to_read
    pub unsafe fn read_bytes(
        &self,
        addr_to_read: usize,
        size_to_read: usize,
    ) -> anyhow::Result<Vec<u8>> {
        let mut buffer_vec: Vec<u8> = vec![0; size_to_read];
        ReadProcessMemory(
            self.process_handle,
            addr_to_read as *const c_void,
            buffer_vec.as_mut_ptr() as _,
            size_to_read,
            None,
        )?;
        Ok(buffer_vec)
    }
    /// Iterate loaded modules and execute function 'f' upon each iteration.
    pub unsafe fn iterate_modules<F>(&self, callback: F) -> anyhow::Result<()>
    where
        F: Fn(MODULEENTRY32),
    {
        // Get module snapshot
        let module_snapshot =
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE32 | TH32CS_SNAPMODULE, self.process_id)?;
        // Setup module struct
        let mut module_entry = MODULEENTRY32::default();
        // Set struct size
        module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
        // Check first module
        if let Err(e) = Module32First(module_snapshot, &mut module_entry) {
            CloseHandle(module_snapshot)?;
            return Err(e.into());
        }
        loop {
            // Execute function f
            callback(module_entry);
            // Clear buffer
            module_entry
                .szModule
                .iter_mut()
                .for_each(|e_byte| *e_byte = 0x0);
            // Onto next entry
            match Module32Next(module_snapshot, &mut module_entry) {
                Ok(_) => continue,
                Err(e) if e.raw_os_error() == Some(ERROR_NO_MORE_FILES as _) => break,
                Err(e) => {
                    CloseHandle(module_snapshot)?;
                    return Err(e.into());
                }
            }
        }
        CloseHandle(module_snapshot)?;
        Ok(())
    }
}
