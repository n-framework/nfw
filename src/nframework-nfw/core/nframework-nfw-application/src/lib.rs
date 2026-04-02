//! Application layer for nframework-nfw.

pub mod features;

// Re-export domain validation utilities for use by presentation layer
pub use nframework_nfw_domain::features::template_management::validation;
