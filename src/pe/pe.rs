use exe::headers::ImageImportDescriptor;
use exe::pe::VecPE;
use exe::types::{CCharString, ImportDirectory};

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
