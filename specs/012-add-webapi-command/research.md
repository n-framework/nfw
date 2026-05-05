# Phase 0: Research

## YAML Comment Preservation in Rust

- **Decision**: Use a regex-based or string-manipulation-based update function tailored for `nfw.yaml` or a specialized crate if a robust parser is needed. Given typical CLI updates, a line-by-line aware editor or specific regex for appending modules is often most robust for preserving formatting and comments in config files where `serde_yaml` natively drops them.
- **Rationale**: `serde_yaml` deserializes into a model and serializes back without comments. To satisfy FR-006 (preserve existing structural comments), the config update must be a surgical AST or string-based patch rather than a full read/write cycle.
- **Alternatives considered**: Migrating from YAML to a comment-preserving format like TOML or JSONC (out of scope for this spec). Writing a custom full-YAML parser (too complex). Using `serde_yaml` and ignoring the requirement (violates FR-006).

## Safe Rollback Mechanism

- **Decision**: Implement a `Transaction` or `RollbackContext` pattern in the generation logic.
- **Rationale**: Keeps track of created files and modified config. If an error occurs, it deletes the tracked files and restores the config from a memory backup.
- **Alternatives considered**: Relying on Git for rollback (not safe if working directory has other uncommitted changes).
