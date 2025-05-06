use exe::headers::ImageDirectoryEntry;
use exe::headers::ImageImportDescriptor;
use exe::types::{CCharString, ImportDirectory};
use exe::Buffer;
use exe::VecPE;
use exe::PE;
use exe::RVA;

/// Structure representing a runtime function entry in the exception directory
///
#[derive(Debug, Clone)]
pub struct RuntimeFunction {
    pub begin_address: u32,
    pub end_address: u32,
    pub unwind_info: u32,
}

/// Get a VecPE from an image path provided
///
pub fn get_pe_from_path(path: String) -> anyhow::Result<VecPE> {
    Ok(VecPE::from_disk_file(path)?)
}

/// Get imports from a specific module in an image
///
pub fn get_imports<'a>(image: &'a VecPE) -> anyhow::Result<ImportDirectory<'a>> {
    Ok(ImportDirectory::parse(image)?)
}

/// Get imports from a specific module in an image
///
pub fn get_imports_descriptor_from_name(
    image: VecPE,
    module_name: String,
) -> anyhow::Result<ImageImportDescriptor> {
    let import_directory = ImportDirectory::parse(&image)?;

    let descriptor =
        import_directory
            .descriptors
            .iter()
            .find(|descriptor| match descriptor.get_name(&image) {
                Ok(name) => match name.as_str() {
                    Ok(str_name) => str_name.to_lowercase() == module_name.to_lowercase(),
                    Err(_) => false,
                },
                Err(_) => false,
            });
    descriptor
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Module {module_name} not found"))
}

/// Get all functions within exception directory
///
pub fn get_functions(image: &VecPE) -> anyhow::Result<Vec<RuntimeFunction>> {
    // In x64 compiled binaries, the exception directory actually has a list of all functions,
    // their start, end & unwind info so that's where we're going ot be looking for functions.
    // there are of course other ways of doing this but for now this is what we'll be using.
    let exception_dir = match image.get_data_directory(ImageDirectoryEntry::Exception) {
        Ok(dir) => dir,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "No exception directory found or it's inaccessible"
            ))
        }
    };

    if exception_dir.size == 0 {
        return Ok(Vec::new());
    }

    // All we have to do is divide the count by the size of the struct, it's 4 bytes per field and
    // we've 3.
    let entry_size = 12;
    let count = exception_dir.size / entry_size;
    let mut functions = Vec::with_capacity(count as usize);

    // Iterate over all and extract necessary information from it.
    for i in 0..count {
        let entry_rva = RVA(exception_dir.virtual_address.0 + (i * entry_size));

        let file_offset = match image.rva_to_offset(entry_rva) {
            Ok(offset) => offset,
            Err(_) => return Err(anyhow::anyhow!("Failed to convert RVA to file offset")),
        };

        let begin_bytes = match image.read(file_offset.0 as usize, 4) {
            Ok(bytes) => bytes,
            Err(_) => return Err(anyhow::anyhow!("Failed to read begin_address")),
        };
        let begin_address = u32::from_le_bytes([
            begin_bytes[0],
            begin_bytes[1],
            begin_bytes[2],
            begin_bytes[3],
        ]);

        let end_bytes = match image.read((file_offset.0 + 4) as usize, 4) {
            Ok(bytes) => bytes,
            Err(_) => return Err(anyhow::anyhow!("Failed to read end_address")),
        };
        let end_address =
            u32::from_le_bytes([end_bytes[0], end_bytes[1], end_bytes[2], end_bytes[3]]);

        let unwind_bytes = match image.read((file_offset.0 + 8) as usize, 4) {
            Ok(bytes) => bytes,
            Err(_) => return Err(anyhow::anyhow!("Failed to read unwind_info")),
        };
        let unwind_info = u32::from_le_bytes([
            unwind_bytes[0],
            unwind_bytes[1],
            unwind_bytes[2],
            unwind_bytes[3],
        ]);

        functions.push(RuntimeFunction {
            begin_address,
            end_address,
            unwind_info,
        });
    }

    Ok(functions)
}
