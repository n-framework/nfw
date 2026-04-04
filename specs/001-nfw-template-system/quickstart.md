# Template Authoring Quickstart

This guide shows you how to create, validate, and publish a template for the NFramework CLI.

## Prerequisites

- Git installed and configured
- Basic understanding of YAML syntax
- A git repository for your template (GitHub, GitLab, etc.)

## Step 1: Create the Template Structure

Create a new directory for your template:

```bash
mkdir my-nfw-template
cd my-nfw-template
```

Create the required structure:

```bash
# Create the content directory
mkdir content

# Create template metadata
touch template.yaml
```

## Step 2: Define Template Metadata

Edit `template.yaml` with your template information:

```yaml
# Template identifier (kebab-case)
id: my-microservice

# Human-readable name
name: My Custom Microservice

# One-line description
description: A .NET microservice with custom features

# Semantic version
version: 1.0.0

# Optional target language (dotnet, go, rust, neutral)
# Omit for language-agnostic templates
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
sourceUrl: https://github.com/yourusername/nfw-templates
```

## Step 3: Create Template Content

Add files to the `content/` directory. These files will be generated when users create a workspace with your template.

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

Before publishing, validate your template structure:

```bash
# Clone your repo locally (or use existing directory)
cd /path/to/my-nfw-template

# Verify structure
ls -la
# Should see: template.yaml, content/

# Verify metadata is valid YAML
# (Manual check: ensure no syntax errors)
```

**Validation Checklist**:

- [ ] `template.yaml` exists and is valid YAML
- [ ] All required fields are present: `id`, `name`, `description`, `version`
- [ ] `id` is kebab-case (lowercase, letters, numbers, hyphens only)
- [ ] `version` follows semantic versioning (e.g., 1.0.0)
- [ ] If provided, `language` is one of: `dotnet`, `go`, `rust`, `neutral`
- [ ] `content/` directory exists and is not empty

## Step 6: Publish to Git Repository

Commit and push your template to a git repository:

```bash
git init
git add template.yaml content/ .nfwignore
git commit -m "Initial template: my-microservice v1.0.0"
git branch -M main
git remote add origin https://github.com/yourusername/nfw-templates.git
git push -u origin main
```

## Step 7: Register Template Source in NFW CLI

Register your template repository with the NFW CLI:

```bash
# Add your template source
nfw templates add https://github.com/yourusername/nfw-templates

# List available templates (should include yours)
nfw templates list
```

## Step 8: Use Your Template

Create a workspace using your template:

```bash
# Create a new workspace with your template
nfw new MyWorkspace --template my-microservice

# Or with qualified identifier if needed
nfw new MyWorkspace --template your-repo/my-microservice
```

## Creating a Template Catalog

To distribute multiple templates in one repository:

```bash
nfw-templates/
├── microservice/
│   ├── template.yaml
│   └── content/
├── worker/
│   ├── template.yaml
│   └── content/
└── grpc-service/
    ├── template.yaml
    └── content/
```

Each subdirectory is a separate template with its own `template.yaml`.

## Troubleshooting

### Template not appearing in list

**Problem**: Your template doesn't show up in `nfw templates list`

**Solutions**:

1. Check that `template.yaml` is valid YAML
2. Verify all required fields are present
3. Ensure the repository URL is accessible
4. Run `nfw templates --refresh` to update cache

### Validation errors

**Problem**: CLI reports validation errors for your template

**Common Issues**:

- Invalid `id`: Use only lowercase letters, numbers, and hyphens
- Invalid `version`: Use semantic versioning (1.0.0, not 1.0)
- Invalid `language`: Use only `dotnet`, `go`, `rust`, or `neutral`
- Missing `content/` directory: Must exist at same level as `template.yaml`

### Cache issues

**Problem**: Changes to template not reflected

**Solution**:

```bash
# Force cache refresh
nfw templates --refresh

# Or clear cache manually
rm -rf ~/.cache/nfw/templates/*
nfw templates --refresh
```

## Next Steps

- Review the [Template Metadata Schema](contracts/template-metadata-schema.yaml) for all available fields
- Review the [Repository Structure](contracts/template-repository-structure.md) for advanced options
- Contribute your template to the official repository: <https://github.com/n-framework/nfw-templates>

## Additional Resources

- [NFramework Documentation](https://github.com/n-framework/n-framework)
- [Template Examples](https://github.com/n-framework/nfw-templates)
- [Issue Tracker](https://github.com/n-framework/n-framework/issues)
