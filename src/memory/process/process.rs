use super::iterators::allocation_iter::Allocation;
use super::iterators::module_iter::ModuleIterator;
use super::iterators::process_iter::ProcessIterator;
use super::ProcessErrors;
use super::{compare_values, convert_value_to_bytes, extract_value};
use crate::gui::windows::scanner_view::scanner_view::ScanResult;
use crate::gui::windows::scanner_view::scanner_view::ScanType;
use crate::gui::windows::scanner_view::scanner_view::ValueType;
use crate::to_rstr;
use std::ffi::c_void;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;
use windows::Win32::System::Memory::*;
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
    /// Read partitions of large allocations
    ///
    pub unsafe fn read_bytes_paged(
        &mut self,
        address: usize,
        size: usize,
    ) -> anyhow::Result<Vec<u8>> {
        let step_size: usize = std::cmp::min(size, 0x1000);
        let mut buffer = Vec::with_capacity(size);

        for i in (0..size).step_by(step_size) {
            let remaining = size - i;
            let size_to_read = std::cmp::min(step_size, remaining);

            let page_result = self.read_bytes(address + i, size_to_read);

            match page_result {
                Ok(page_data) => {
                    buffer.extend_from_slice(&page_data);
                }
                Err(_) => {
                    // If a page fails, fill with zeros and continue
                    buffer.extend_from_slice(&vec![0u8; size_to_read]);
                }
            }
        }

        Ok(buffer)
    }
    /// Search through allocations for specific data
    ///
    pub unsafe fn find_data<T>(
        &mut self,
        scan_type: ScanType,
        value_type: ValueType,
        value: T,
        fast_scan: bool,
    ) -> anyhow::Result<Vec<ScanResult>>
    where
        T: Clone + std::fmt::Debug + 'static,
    {
        // Our return result
        let mut result = Vec::new();
        // So we're going to want to take in three parameters, fast or slow scan as well as what type
        // we're searching for and lastly the value we're looking for
        let allocations: Vec<MEMORY_BASIC_INFORMATION> = Allocation::new(self)?.collect();

        // Get our value into bytes for comparison
        let value_bytes = convert_value_to_bytes(&value, &value_type)?;

        // Set the step size depending on the valuetype
        let step_size = match value_type {
            ValueType::Byte => 1,
            ValueType::TwoBytes => 2,
            ValueType::FourBytes => 4,
            ValueType::EightBytes => 8,
        };

        // Iterate over all allocations now
        for alloc in allocations.iter() {
            // Skip if region has unsuitable protection (e.g., no read access)
            if alloc.Protect.0 & (PAGE_NOACCESS.0 | PAGE_GUARD.0) != 0 {
                continue;
            }

            // Only scan regions that have read access
            if alloc.Protect.0
                & (PAGE_READONLY.0
                    | PAGE_READWRITE.0
                    | PAGE_EXECUTE_READ.0
                    | PAGE_EXECUTE_READWRITE.0)
                == 0
            {
                continue;
            }
            let alloc_bytes =
                self.read_bytes_paged(alloc.BaseAddress as _, alloc.RegionSize as _)?;
            // Iterate through the allocation now
            for idx in (0..alloc_bytes.len()).step_by(if fast_scan { step_size } else { 1 }) {
                if idx + step_size > alloc_bytes.len() {
                    break;
                }

                // Check if the value matches
                let is_match = compare_values(
                    &alloc_bytes[idx..idx + step_size],
                    &value_bytes,
                    &scan_type,
                    &value_type,
                )?;

                if is_match {
                    let found_value =
                        extract_value(&alloc_bytes[idx..idx + step_size], &value_type);

                    result.push(ScanResult::new(
                        (alloc.BaseAddress as u64) + idx as u64,
                        found_value,
                    ));
                }
            }
        }

        Ok(result)
    }
}
