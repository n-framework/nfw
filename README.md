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

## Templates Source Behavior

- Debug builds use `packages/nfw-templates` when the submodule exists.
- If the debug submodule is missing, debug builds fall back to release behavior.
- Release builds fetch templates from `github.com/n-framework/nfw-templates`
  at tag `v{cliVersion}`.

Template source is not user-configurable in the CLI skeleton phase.

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
