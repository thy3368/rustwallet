pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

// Re-export commonly used types
pub use domain::{Address, Balance, DomainError, Network};
