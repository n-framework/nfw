# Quickstart: Template-Based `nfw add service`

## 1. Add a service in non-interactive mode

```bash
nfw add service Orders --template official/dotnet-service --no-input
```

Expected behavior:

- Generates service at `src/Orders/`
- Creates layer projects: `Domain`, `Application`, `Infrastructure`, `Api`
- Renders template-defined API baseline (including health endpoints for official template)
- Persists template provenance in `nfw.yaml`

## CLI help contract

```bash
nfw add service --help
```

Expected output includes:

- `Usage: nfw add service <name> [OPTIONS]`
- `--template <template>`
- `--no-input`

## 2. Add a service in interactive mode

```bash
nfw add service Orders
```

Expected behavior:

- Prompts for template when omitted
- Resolves template/version before writes
- Generates service only after validation is complete

## 3. Verify generated structure quickly

```bash
ls src/Orders
```

Expected directories/files include layer projects and template-defined artifacts for:

- `Domain`
- `Application`
- `Infrastructure`
- `Api`

## 4. Verify build and template baseline

Run workspace-documented one-command build and test flows, then verify template-rendered API baseline:

```bash
rg -n "health/live|health/ready" src/Orders
```

Expected behavior: official service template output includes both health endpoint mappings.

## 5. Verify strict failure cases

### Case A: Missing template in non-interactive mode

`nfw add service Orders --no-input` must fail before generation with actionable template guidance.

### Case B: Wrong template type

If template metadata is not `type=service`, command must fail before rendering.

### Case C: Existing service directory conflict

If `src/Orders/` already exists, command must fail before writes.

## 6. Acceptance verification commands

Commands expected for implementation verification, runnable from repository root:

```bash
make -C src/nfw build
make -C src/nfw test
make -C src/nfw format
make -C src/nfw lint
```

Verification status (executed on 2026-04-04):

- `make -C src/nfw build` ✅
- `make -C src/nfw test` ✅
- `make -C src/nfw format` ✅
- `make -C src/nfw lint` ✅
