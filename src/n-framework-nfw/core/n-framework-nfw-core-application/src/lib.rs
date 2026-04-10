//! Application layer for nframework-nfw.

pub mod features;

// Re-export domain validation utilities for use by presentation layer
pub use n_framework_nfw_core_domain::features::template_management::validation;
