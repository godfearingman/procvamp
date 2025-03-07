use exe::pe::{VecPE, PE};
use exe::types::{CCharString, ImageImportDescriptor, ImportData, ImportDirectory};

/// Get a VecPE from an image path provided
///
pub fn get_pe_from_path(path: String) -> anyhow::Result<VecPE> {
    Ok(VecPE::from_disk_file(path)?)
}

/// Get imports from a specific module in an image
///
pub fn get_imports_discriptor_from_name<'a>(
    image: &'a VecPE,
    module_name: String,
) -> anyhow::Result<&'a ImageImportDescriptor> {
    let import_directory = ImportDirectory::parse(image)?;

    let descriptor =
        import_directory
            .descriptors
            .iter()
            .find(|descriptor| match descriptor.get_name(image) {
                Ok(name) => match name.as_str() {
                    Ok(str_name) => str_name.to_lowercase() == module_name.to_lowercase(),
                    Err(_) => false,
                },
                Err(_) => false,
            });
    descriptor.ok_or_else(|| anyhow::anyhow!("Module not found"))
}
