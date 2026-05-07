# Quickstart: nfw add webapi

## Adding a WebAPI Module Interactively

Run the command in your workspace root:

```bash
nfw add webapi
```

Select the target service from the interactive prompt. The CLI will generate the Minimal API layer, attach the necessary middleware (CORS, OpenAPI, Problem Details, Health Checks), and register it in your `nfw.yaml`.

## Adding via Automation

For CI/CD or template scripts, pass the service name and disable inputs:

```bash
nfw add webapi --service Catalog --no-input
```

## Rollback Guarantee

If the generation process encounters any error (e.g., missing template, file permission issue), all generated files are safely cleaned up and your `nfw.yaml` is restored to its original state, including all comments.
