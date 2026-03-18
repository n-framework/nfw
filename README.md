# nfw CLI

The `nfw` CLI is the command-line entry point for the n-framework toolchain.

## Developer Workflow

Run commands from `src/nfw`:

```bash
make build
make test
make format
make analyze
```

The workflow uses one-step build and test commands to keep local and CI usage
consistent.

From the repository root, the equivalent wrapper commands are:

```bash
./scripts/build.sh
./scripts/test.sh
```

## Templates Source Behavior

- Debug builds use `packages/nfw-templates` when the submodule exists.
- If the debug submodule is missing, debug builds fall back to release behavior.
- Release builds fetch templates from `github.com/n-framework/nfw-templates`
  at tag `v{cliVersion}`.

Template source is not user-configurable in the CLI skeleton phase.

## Template Catalog

`nfw templates` lists stable template identifiers that are safe to reuse in
automation:

- `blank` - Blank Workspace
- `minimal` - Minimal API

The command renders identifier, display name, and description in catalog order.

## Workspace Creation

`nfw new` now accepts an optional positional workspace name. When it is missing
and the terminal is interactive, the CLI prompts for it.

Use an explicit identifier when running in CI or any non-interactive context:

```bash
dotnet run --project src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new sample-explicit --template blank
```

Disable all interactive questions explicitly with `--no-input`:

```bash
dotnet run --project src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new --no-input sample-explicit --template blank
```

If the terminal is interactive and `--template` is omitted, `nfw new` prompts
for a selection from the same catalog metadata shown by `nfw templates`. If the
workspace name is also omitted, the CLI prompts for that first.

If the terminal is non-interactive, or if `--no-input` is passed, the command
fails with a usage error before creating files whenever required inputs are
missing.

## Configuration

- Optional config file: `nfw.yaml` in the current working directory.
- Environment variables with `NFW_` prefix override file keys.
- Invalid YAML prints an error and the CLI continues with defaults.

## Exit Codes

- `0` success, including help output.
- `1` runtime failure.
- `2` usage error (unknown command/flag, invalid args).
- `130` interrupted by SIGINT.

## Publish

Single-file publish examples:

```bash
dotnet publish src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -c Release -r linux-x64 -p:PublishSingleFile=true
dotnet publish src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -c Release -r osx-x64 -p:PublishSingleFile=true
dotnet publish src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -c Release -r win-x64 -p:PublishSingleFile=true
```
