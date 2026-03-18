# Quickstart: Template Catalog Listing and Selection

## Prerequisites

- Work from `/home/ac/Code/n-framework/n-framework`
- Use the feature branch `002-nfw-template-catalog-selection`
- Ensure the .NET SDK pinned by the repo is installed

## Build

```bash
cd /home/ac/Code/n-framework/n-framework
./scripts/build.sh
```

## Test

```bash
cd /home/ac/Code/n-framework/n-framework
./scripts/test.sh
```

## Manual Verification

### 1. List templates

```bash
cd /home/ac/Code/n-framework/n-framework
dotnet run --project /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- templates
```

Expected result:

- Command exits with `0`
- Output is written to stdout
- Each template row shows identifier, display name, and description in deterministic order
- The local debug catalog lists `blank` and `minimal`

### 2. Create a workspace with an explicit template

```bash
cd /tmp
dotnet run --project /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new sample-explicit --template blank
```

Expected result:

- Command exits with `0`
- No prompt is shown
- The selected template is `blank`
- `sample-explicit/nfw.yaml` records `template: blank`

### 3. Create a workspace interactively

```bash
cd /tmp
dotnet run --project /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new
```

Expected result:

- When run in a real terminal, the CLI prompts for the missing workspace name first
- When run in a real terminal, the CLI prompts for template selection
- Prompt choices match the order and metadata shown by `nfw templates`
- Interrupting the prompt leaves no partially created workspace behind

### 4. Verify non-interactive failure without required explicit inputs

```bash
cd /tmp
printf '' | dotnet run --project /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new --no-input
```

Expected result:

- Command exits with `2`
- Error text is written to stderr
- No workspace directory is created

### 5. Verify explicit non-interactive success with prompts disabled

```bash
cd /tmp
dotnet run --project /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new --no-input sample-explicit-no-input --template blank
```

Expected result:

- Command exits with `0`
- No prompt is shown
- `sample-explicit-no-input/nfw.yaml` records `template: blank`
