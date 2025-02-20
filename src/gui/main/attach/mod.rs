pub mod attach;

/// Create an enum type for our selectable label for process list
///
#[derive(PartialEq)]
enum ProcessEnum {
    Title(String),
}
