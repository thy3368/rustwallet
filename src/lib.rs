pub mod infrastructure;
pub mod interfaces;

pub mod core;

// Re-export commonly used types
pub use core::domain::{Address, Balance, DomainError, Network};
