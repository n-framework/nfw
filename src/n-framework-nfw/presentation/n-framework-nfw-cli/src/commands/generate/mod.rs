//! Template-based code generation command implementation.
//!
//! This module provides the `nfw generate` command, which allows developers to
//! scaffold code components from templates using a configuration-driven approach.

pub mod command;
pub use command::{GenerateCliCommand, GenerateError, GenerateRequest};
