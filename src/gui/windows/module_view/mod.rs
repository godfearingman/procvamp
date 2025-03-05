pub mod module_view;

/// Create an enum type for our selectable label for module list
///
#[derive(PartialEq, Clone)]
pub enum ModuleEnum {
    Title(String),
}
