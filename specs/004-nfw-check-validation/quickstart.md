# Quickstart: `nfw check` Architecture Validation

## Prerequisites

- NFramework workspace available locally.
- `nfw` CLI built and runnable.

## Build and Test from Repository Root

1. Build with one command from repo root:
   - `make -C src/nfw build`
2. Run full module tests with one command from repo root:
   - `make -C src/nfw test`

## Validate a Clean Workspace

1. From workspace root, run:
   - `nfw check`
2. Expected result:
   - Success message.
   - Exit code `0`.
   - `make lint` is executed by `nfw check` as part of the validation run.
   - `make test` is also executed for each service declared in `nfw.yaml` under `services.*.path`.
   - Example output:
     - `architecture validation passed in '<workspace-root>': no forbidden project references, namespaces, direct packages, lint issues, or service test issues found`

## Validate Violation Detection with Fixtures

1. Run fixture-backed tests for architecture validation in the `src/nfw` module test suite.
2. Confirm expected behavior:
   - Valid fixture exits `0` with no findings.
   - Invalid fixtures exit non-zero and include actionable findings.
   - Lint failures from `make lint` are reported as `lint_issue` findings.
   - Service `make test` failures are reported as `test_issue` findings.
   - Unreadable-artifact fixture exits non-zero while still reporting other readable findings.
   - Example failure line:
     - `- type=project_reference location=<path> offending=<reference> hint=<remediation>`

## CI Usage

- Add `nfw check` as a required non-interactive CI step.
- Treat any non-zero exit as a merge-blocking failure until findings are remediated.
- Validate linting from repo root as part of CI quality gates:
  - `make -C src/nfw lint`
