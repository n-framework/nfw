# Quickstart: Template-Based `nfw add service`

## 1. Add a service in non-interactive mode

```bash
nfw add service Orders --template official/dotnet-service --no-input
```

Expected behavior:

- Generates service at `src/Orders/`
- Creates layer projects: `Domain`, `Application`, `Infrastructure`, `Api`
- Scaffolds health endpoints in API layer
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

## 4. Verify build and health baseline

Run workspace-documented one-command build and test flows, then start generated API and verify:

```bash
curl -i http://localhost:<port>/health/live
curl -i http://localhost:<port>/health/ready
```

Expected behavior: both return HTTP `200`.

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
