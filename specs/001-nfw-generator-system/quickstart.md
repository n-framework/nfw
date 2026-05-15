# Generator Authoring Quickstart

This guide shows you how to create, validate, and publish a generator for the NFramework CLI.

## Prerequisites

- Git installed and configured
- Basic understanding of YAML syntax
- A git repository for your generator (GitHub, GitLab, etc.)

## Step 1: Create the Generator Structure

Create a new directory for your generator:

```bash
mkdir my-nfw-generator
cd my-nfw-generator
```

Create the required structure:

```bash
# Create the content directory
mkdir content

# Create generator metadata
touch nfw.generator.yaml
```

## Step 2: Define Generator Metadata

Edit `nfw.generator.yaml` with your generator information:

```yaml
# Generator identifier (kebab-case)
id: my-microservice

# Human-readable name
name: My Custom Microservice

# One-line description
description: A .NET microservice with custom features

# Semantic version
version: 1.0.0

# Optional target language (dotnet, go, rust, neutral)
# Omit for language-agnostic generators
language: dotnet

# Optional: Searchable tags
tags:
  - microservice
  - web-api

# Optional: Author
author: Your Name <you@example.com>

# Optional: Minimum CLI version
minCliVersion: 0.1.0

# Optional: Canonical repository
sourceUrl: https://github.com/yourusername/nfw-generators
```

## Step 3: Create Generator Content

Add files to the `content/` directory. These files will be generated when users create a workspace with your generator.

```bash
# Example structure
content/
├── src/
│   ├── __ServiceName__.WebApi/
│   │   └── Controllers/
│   ├── __ServiceName__.Application/
│   └── __ServiceName__.Domain/
└── __ServiceName__.sln
```

### Using Placeholders

Replace dynamic values with `__PlaceholderName__` patterns:

- `__ServiceName__` → Replaced with the service name
- `__Namespace__` → Replaced with the project namespace
- `__ProjectGuid__` → Replaced with a generated GUID

**Example**: A file named `__ServiceName__.cs` becomes `Orders.cs` when the user creates a service named "Orders".

## Step 4: (Optional) Define Exclusions

Create `.nfwignore` to exclude files from generation:

```gitignore
# Exclude documentation
README.md
docs/

# Exclude git files
.git/

# Exclude IDE files
.vscode/
.idea/
```

## Step 5: Validate Locally

Before publishing, validate your generator structure:

```bash
# Clone your repo locally (or use existing directory)
cd /path/to/my-nfw-generator

# Verify structure
ls -la
# Should see: nfw.generator.yaml, content/

# Verify metadata is valid YAML
# (Manual check: ensure no syntax errors)
```

**Validation Checklist**:

- [ ] `nfw.generator.yaml` exists and is valid YAML
- [ ] All required fields are present: `id`, `name`, `description`, `version`
- [ ] `id` is kebab-case (lowercase, letters, numbers, hyphens only)
- [ ] `version` follows semantic versioning (e.g., 1.0.0)
- [ ] If provided, `language` is one of: `dotnet`, `go`, `rust`, `neutral`
- [ ] `content/` directory exists and is not empty

## Step 6: Publish to Git Repository

Commit and push your generator to a git repository:

```bash
git init
git add nfw.generator.yaml content/ .nfwignore
git commit -m "Initial generator: my-microservice v1.0.0"
git branch -M main
git remote add origin https://github.com/yourusername/nfw-generators.git
git push -u origin main
```

## Step 7: Register Generator Source in NFW CLI

Register your generator repository with the NFW CLI:

```bash
# Add your generator source
nfw generators add https://github.com/yourusername/nfw-generators

# List available generators (should include yours)
nfw generators list
```

## Step 8: Use Your Generator

Create a workspace using your generator:

```bash
# Create a new workspace with your generator
nfw new MyWorkspace --generator my-microservice

# Or with qualified identifier if needed
nfw new MyWorkspace --generator your-repo/my-microservice
```

## Creating a Generator Catalog

To distribute multiple generators in one repository:

```bash
nfw-generators/
├── microservice/
│   ├── nfw.generator.yaml
│   └── content/
├── worker/
│   ├── nfw.generator.yaml
│   └── content/
└── grpc-service/
    ├── nfw.generator.yaml
    └── content/
```

Each subdirectory is a separate generator with its own `nfw.generator.yaml`.

## Troubleshooting

### Generator not appearing in list

**Problem**: Your generator doesn't show up in `nfw generators list`

**Solutions**:

1. Check that `nfw.generator.yaml` is valid YAML
2. Verify all required fields are present
3. Ensure the repository URL is accessible
4. Run `nfw generators --refresh` to update cache

### Validation errors

**Problem**: CLI reports validation errors for your generator

**Common Issues**:

- Invalid `id`: Use only lowercase letters, numbers, and hyphens
- Invalid `version`: Use semantic versioning (1.0.0, not 1.0)
- Invalid `language`: Use only `dotnet`, `go`, `rust`, or `neutral`
- Missing `content/` directory: Must exist at same level as `nfw.generator.yaml`

### Cache issues

**Problem**: Changes to generator not reflected

**Solution**:

```bash
# Force cache refresh
nfw generators --refresh

# Or clear cache manually
rm -rf ~/.cache/nfw/generators/*
nfw generators --refresh
```

## Next Steps

- Review the [Generator Metadata Schema](contracts/generator-metadata-schema.yaml) for all available fields
- Review the [Repository Structure](contracts/generator-repository-structure.md) for advanced options
- Contribute your generator to the official repository: <https://github.com/n-framework/nfw-generators>

## Additional Resources

- [NFramework Documentation](https://github.com/n-framework/n-framework)
- [Generator Examples](https://github.com/n-framework/nfw-generators)
- [Issue Tracker](https://github.com/n-framework/n-framework/issues)
