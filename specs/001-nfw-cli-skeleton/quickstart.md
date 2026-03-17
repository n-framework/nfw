# Quickstart: nfw CLI Skeleton

This quickstart is intended for the CLI implementation that follows this plan.

## Prerequisites

- .NET SDK 11.x installed (`dotnet --version`)

## Build

From repository root:

```bash
# Only needed once the n-framework-core-codegen submodule is added.
git submodule update --init --recursive
dotnet build src/nfw/NFramework.Nfw.slnx
```

## Run (Development)

Once the CLI project exists at `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj`:

```bash
dotnet run --project src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- --help
dotnet run --project src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- --version
dotnet run --project src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- templates
```

## Test

```bash
dotnet test src/nfw/NFramework.Nfw.slnx
```

## Format & Analysis (Dev Workflow)

```bash
dotnet tool restore
dotnet csharpier .
dotnet roslynator analyze
```

## Configuration

- Create `nfw.yaml` in the working directory (optional in skeleton phase).
- Override keys via environment variables: `NFW_<SETTING_NAME>`.
