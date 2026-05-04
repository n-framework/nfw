# Quickstart: Generate Repository Command

This guide shows you how to use the `nfw gen repository` command to automate the generation of repository interfaces, implementations, and DI registrations.

## Prerequisites

1. A service workspace created with `nfw new`
2. At least one entity generated with `nfw gen entity <Name>`
3. The persistence module added to the service with `nfw add persistence`

## Basic Usage

The most common way to generate a repository is using the interactive mode:

```bash
nfw gen repository
```

The CLI will prompt you for:

1. The name of the entity (e.g., `User`, `Product`)
2. The feature module where the entity resides (auto-detected if possible)

## Auto-Detection

If you provide the entity name, the CLI will attempt to auto-detect its feature:

```bash
nfw gen repository User
```

If the `User` entity is found uniquely in the `identity` feature, the command will automatically proceed without prompting for the feature name.

## Explicit Feature Target

If you have identically named entities in different features, or want to bypass auto-detection, use the `--feature` flag:

```bash
nfw gen repository Order --feature payments
```

## Non-Interactive Mode

For CI/CD environments or scripting, disable interactive prompts using `--no-input`:

```bash
nfw gen repository Product --feature catalog --no-input
```

## Multi-Service Workspaces

If your workspace contains multiple services, you must specify which service to target using the `--service` flag:

```bash
nfw gen repository Customer --feature crm --service CustomerService
```

## Generated Artifacts

Running the command will produce three modifications:

1. **Interface**: `src/core/<Service>.Core.Application/Features/<Feature>/Repositories/I<Entity>Repository.cs`
2. **Implementation**: `src/infrastructure/<Service>.Infrastructure.Persistence/Features/<Feature>/Repositories/<Entity>Repository.cs`
3. **DI Registration**: Injects `services.AddScoped<I<Entity>Repository, <Entity>Repository>();` into `src/infrastructure/<Service>.Infrastructure.Persistence/ServiceRegistration.cs`
