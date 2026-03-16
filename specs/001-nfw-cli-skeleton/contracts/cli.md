# Contract: nfw CLI (Skeleton Phase)

## Overview

This contract defines the stable command and flag surface for the initial `nfw` CLI skeleton.

## Global Behavior

- Output: Normal output goes to stdout.
- Diagnostics and errors: go to stderr. When `--verbose` is set, include extra diagnostic detail.
- Config discovery: `nfw.yaml` in the current working directory is loaded if present.
- Config precedence: environment variables override file keys for the same configuration key.
- Interrupt handling: SIGINT (Ctrl+C) exits with code `130`.

## Exit Codes

| Code | Meaning                                                   |
| ---: | --------------------------------------------------------- |
|    0 | Success (including `--help` output and no-args help)      |
|    1 | Runtime failure (unexpected exception, IO failures, etc.) |
|    2 | Usage error (unknown command/flag, invalid arguments)     |
|  130 | Interrupted (SIGINT)                                      |

## Commands

### `nfw` (no args)

Shows top-level help.

### `nfw --help` / `nfw -h`

Shows top-level help.

### `nfw --version` / `nfw -v`

Shows version info.

### `nfw templates`

Lists available templates.

- Template source is not user-configurable.
- Debug builds: read templates from `src/nfw/packages/n-framework-nfw-templates` (git submodule) if present; otherwise behave like Release.
- Release builds: fetch templates from `github.com/n-framework/nfw-templates` using the release tag `v{cliVersion}`.
- If no templates are available, print a friendly “no templates available” message (not an error).
- If the remote repository is unreachable or returns an invalid response, print a clear error message and exit `1`.
- Each template should include a name and brief description when available.

Template catalog schema (`catalog.yaml`):

```yaml
templates:
  - name: workspace-basic
    description: Basic workspace starter template
```

- `templates` is optional; when missing or empty, output should indicate no templates are available.
- Each item must contain `name`; missing `description` is displayed as `No description provided.`.

## Options

| Option            | Applies To              | Meaning                          |
| ----------------- | ----------------------- | -------------------------------- |
| `--help`, `-h`    | global and all commands | Show help                        |
| `--version`, `-v` | global                  | Show version                     |
| `--verbose`       | global                  | Enable diagnostic stderr logging |

## Error Messaging

- Unknown command (e.g., `nfw hekp`): print an error, suggest closest known commands, and exit `2`.
- Unknown flag: print an error listing valid flags for that command and exit `2`.
