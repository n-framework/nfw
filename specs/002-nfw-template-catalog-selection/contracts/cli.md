# CLI Contract: Template Catalog Listing and Selection

## Scope

This contract defines the user-visible behavior for template listing and template resolution in `/home/ac/Code/n-framework/n-framework/src/nfw`.

## Command: `nfw templates`

### Purpose

List all available workspace templates using stable identifiers that can be reused in later commands.

### Success Behavior

- Exit code: `0`
- Normal output: stdout only
- Output includes one row per available template with:
  - stable template identifier
  - display name
  - short description
- Rows appear in the validated catalog order.

### Empty Catalog Behavior

- Exit code: `0`
- Normal output: stdout only
- Output communicates that no templates are available.

### Failure Behavior

- Exit code: `1` for catalog retrieval or parsing failures
- Error details: stderr only
- Error text must tell the user what failed and preserve a path to diagnosis through existing verbose/diagnostic mechanisms

## Command: `nfw new [workspace-name]`

### Template Resolution Matrix

| Session Type                  | `--template` Provided | Expected Behavior                                                                                      | Exit Code      |
| ----------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------ | -------------- |
| Interactive terminal          | Yes                   | Resolve the provided identifier and continue without prompting for the template                        | `0` on success |
| Interactive terminal          | No                    | Prompt the user to choose from the available templates before generating files                         | `0` on success |
| Non-interactive execution     | Yes                   | Resolve the provided identifier and continue without prompting                                         | `0` on success |
| Non-interactive execution     | No                    | Fail before generating files with an actionable usage message                                          | `2`            |
| Any session with `--no-input` | Yes                   | Continue without prompting and require all remaining command inputs to be provided explicitly          | `0` on success |
| Any session with `--no-input` | No                    | Fail before generating files with an actionable usage message for the missing explicit template choice | `2`            |

### Interactive Prompt Contract

- Prompting is allowed only when both input and output are attached to a terminal.
- Missing required positional inputs may also be prompted in the same interactive session.
- The prompt must show the same identifier, display name, and description metadata exposed by `nfw templates`.
- Prompt order must match the `nfw templates` listing order.
- If the user interrupts the prompt, the command exits without generating workspace files.

### Explicit Identifier Contract

- The `--template <identifier>` value is the public machine-typed selector for a template.
- The `--no-input` flag disables all interactive prompts, including workspace-name questions.
- Identifier matching may be normalized for user convenience, but all confirmation and error messages must use the canonical catalog identifier.
- Unknown or unavailable identifiers fail before file generation begins.

### Output and Error Contract

- Normal progress, prompts, and success confirmation use stdout.
- Usage errors and runtime failures use stderr.
- Missing or unknown template identifiers are usage errors with exit code `2`.
- Empty catalogs during `nfw new` are runtime failures with exit code `1`.
- Interruptions return exit code `130`.
- Catalog retrieval or parse failures return exit code `1`.

### File-System Safety Contract

- Template resolution completes before workspace files are written.
- On missing identifier, unknown identifier, empty catalog, or interruption, the command leaves no partially generated workspace behind.
- Successful workspace creation writes `nfw.yaml` with the selected canonical template identifier plus a starter `README.md` in the new workspace directory.
