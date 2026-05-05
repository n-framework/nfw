# CLI Interface Contract

## Command: `nfw add webapi`

**Description**: Adds a WebAPI module to an existing service.

### Arguments & Flags

- `--service <NAME>` (Optional): Specifies the target service name. If omitted and terminal is interactive, prompts the user.
- `--no-input` (Optional): Disables interactive prompts. If `--service` is not provided, the command fails.

### Exit Codes

- `0`: Success
- `1`: Validation error (e.g., service not found, WebAPI already exists)
- `2`: Generation error (triggers rollback)
- `130`: Interrupted by user (Ctrl+C)

### Interactive Prompts (if applicable)

- "Select the target service:" -> Displays a list of existing services found in `nfw.yaml`.
