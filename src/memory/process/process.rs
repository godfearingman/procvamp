use super::iterators::module_iter::ModuleIterator;
use super::iterators::process_iter::ProcessIterator;
use super::ProcessErrors;
use crate::to_rstr;
use std::ffi::c_void;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

#[derive(Debug, Clone)]
pub struct Process {
    process_name: String,
    process_handle: HANDLE,
    process_id: u32,
    process_base: u64,
}

/// Implement the Drop type for our process struct to automatically close handle on destruction
///
impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            if !self.process_handle.is_invalid() {
                let _ = self.close_handle();
            }
        }
    }
}

impl Process {
    /// Constructor to get all running processes as a list
    ///
    pub unsafe fn get_processes() -> anyhow::Result<Vec<Self>> {
        ProcessIterator::new()?
            .map(|entry| {
                // Get process name
                let proc_name: String = to_rstr!(entry.szExeFile);

                Ok(Self {
                    process_name: proc_name,
                    process_handle: HANDLE::default(),
                    process_id: entry.th32ProcessID,
                    process_base: u64::default(),
                })
            })
            .collect()
    }
    /// Get running modules under a process as a list
    ///
    pub unsafe fn get_modules(&self) -> anyhow::Result<Vec<MODULEENTRY32>> {
        Ok(ModuleIterator::new(self.process_id)?.collect())
    }
    /// Constructor to find a process by name and extract all the key information we'll need to perform analysis on said process
    ///
    pub unsafe fn find(name_of_process: &str) -> anyhow::Result<Self> {
        ProcessIterator::new()?
            .find(|entry| to_rstr!(entry.szExeFile) == name_of_process)
            .map(|entry| {
                // Get process name
                let proc_name: String = to_rstr!(entry.szExeFile);

                Ok(Self {
                    process_name: proc_name,
                    process_handle: HANDLE::default(),
                    process_id: entry.th32ProcessID,
                    process_base: u64::default(),
                })
            })
            .ok_or_else(|| ProcessErrors::ProcessNotFound {
                process_name: name_of_process.to_string(),
            })?
    }
    /// Write value of type T to the given process at location addr_to_write
    ///
    pub unsafe fn write<T>(
        &mut self,
        addr_to_write: usize,
        value_to_write: T,
    ) -> anyhow::Result<()> {
        let handle = self.get_handle()?;
        WriteProcessMemory(
            handle,
            addr_to_write as *const c_void,
            &value_to_write as *const T as *const c_void,
            std::mem::size_of::<T>(),
            None,
        )?;
        Ok(())
    }
    /// Write an array of bytes to the given process at location addr_to_write
    ///
    pub unsafe fn write_bytes(
        &mut self,
        addr_to_write: usize,
        value_to_write: &[u8],
    ) -> anyhow::Result<()> {
        let handle = self.get_handle()?;
        let val_ptr = value_to_write.as_ptr() as *const c_void;
        WriteProcessMemory(
            handle,
            addr_to_write as *const c_void,
            val_ptr,
            std::mem::size_of_val(value_to_write),
            None,
        )?;
        Ok(())
    }
    /// Read memory of type T from the process at the given location addr_to_read
    ///
    pub unsafe fn read<T /*: std::fmt::Display*/>(
        &mut self,
        addr_to_read: usize,
    ) -> anyhow::Result<T> {
        let handle = self.get_handle()?;
        let mut buffer_vec: Vec<u8> = vec![0; std::mem::size_of::<T>()];
        ReadProcessMemory(
            handle,
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
    ///
    pub unsafe fn read_bytes(
        &mut self,
        addr_to_read: usize,
        size_to_read: usize,
    ) -> anyhow::Result<Vec<u8>> {
        let handle = self.get_handle()?;
        let mut buffer_vec: Vec<u8> = vec![0; size_to_read];
        ReadProcessMemory(
            handle,
            addr_to_read as *const c_void,
            buffer_vec.as_mut_ptr() as _,
            size_to_read,
            None,
        )?;
        Ok(buffer_vec)
    }
    /// Open or return an open handle to a targeted process
    ///
    pub unsafe fn get_handle(&mut self) -> anyhow::Result<HANDLE> {
        // Check if fields are valid
        if self.process_id == 0 {
            Err(ProcessErrors::InvalidProcessFields)?
        }

        // Check if handle is already open
        if !self.process_handle.is_invalid() {
            return Ok(self.process_handle);
        }

        // Open and return otherwise
        self.process_handle = OpenProcess(PROCESS_ALL_ACCESS, false, self.process_id)?;
        Ok(self.process_handle)
    }
    /// Close an open handle
    ///
    pub unsafe fn close_handle(&mut self) -> anyhow::Result<()> {
        // Check if called with no selected process
        if self.process_id == 0 {
            Err(ProcessErrors::InvalidProcessFields)?
        }

        // Check if handle is already closed
        if self.process_handle.is_invalid() {
            Err(ProcessErrors::InvalidHandle)?
        }

        // Close otherwise and overwrite handle to default again
        CloseHandle(self.process_handle)?;
        self.process_handle = HANDLE::default();
        Ok(())
    }
    /// Return process name
    ///
    pub fn name(&self) -> String {
        self.process_name.clone()
    }
    /// Return process id
    ///
    pub fn pid(&self) -> u32 {
        self.process_id
    }
    /// Return process base
    ///
    pub unsafe fn base(&mut self) -> anyhow::Result<u64> {
        if self.process_base == u64::default() {
            self.process_base = ModuleIterator::new(self.process_id)?
                .find(|module| to_rstr!(module.szModule) == self.process_name.clone())
                .map(|module| module.modBaseAddr as u64)
                .ok_or_else(|| ProcessErrors::ModuleNotFound {
                    module_name: self.process_name.clone(),
                })?;
        }
        Ok(self.process_base)
    }
}
