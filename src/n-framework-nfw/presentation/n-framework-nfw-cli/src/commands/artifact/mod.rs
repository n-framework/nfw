//! Artifact scaffolding from templates.
//!
//! This module provides the `nfw add <artifact>` commands, which allow developers to
//! scaffold code components from templates using a configuration-driven approach.

pub mod add_artifact;
pub use add_artifact::{AddArtifactCliCommand, AddArtifactError, AddArtifactRequest};
