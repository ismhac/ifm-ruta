//! Core traits and interfaces

pub mod tool;
pub mod command;
pub mod settings;
pub mod process;
pub mod event;
pub mod validation;

// Re-export all traits
pub use tool::*;
pub use command::*;
pub use settings::*;
pub use process::*;
pub use event::*;
pub use validation::*;
