pub mod commands;
pub mod package;
pub mod ui;

// Re-export main types for easier testing
pub use package::{Package, search_packages};
pub use ui::{App, InputMode, ConfirmAction};
