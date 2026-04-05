# Research: Template-Based `nfw add service`

## Decision 1: Template Selection Policy

- **Decision**: Require `--template <id>` in non-interactive mode; prompt for template selection in interactive mode when omitted.
- **Rationale**: Keeps automation deterministic and prevents silent default drift while preserving interactive usability.
- **Alternatives considered**:
  - Always auto-select default template (rejected: hidden behavior in CI)
  - Always require `--template`, even interactive (rejected: weaker UX for local usage)

## Decision 2: Service Output Path

- **Decision**: Always generate service into `src/<ServiceName>/`.
- **Rationale**: Aligns with existing workspace layout conventions and keeps discovery predictable.
- **Alternatives considered**:
  - `src/services/<ServiceName>/` (rejected: extra hierarchy not currently standardized)
  - Template-defined arbitrary root paths (rejected: weaker cross-template consistency)

## Decision 3: Service Template Eligibility

- **Decision**: `nfw add service` accepts only templates explicitly marked `type=service`.
- **Rationale**: Prevents applying incompatible template types and catches mistakes before rendering.
- **Alternatives considered**:
  - Accept any template and validate output shape later (rejected: late failures)
  - Accept any template with warnings (rejected: non-deterministic UX)

## Decision 4: Provenance Persistence

- **Decision**: Persist selected template ID and resolved template version per generated service in workspace `nfw.yaml`.
- **Rationale**: Improves traceability and supports deterministic regeneration audits.
- **Alternatives considered**:
  - Log only to stdout/stderr (rejected: not durable)
  - Store in per-service sidecar file (rejected: fragmented metadata model)

## Decision 5: Dependency Rule Guarantee Strategy

- **Decision**: Treat layer dependency rules as a generation contract and validate generated project references in integration tests.
- **Rationale**: Prevents contract drift from template edits and gives immediate enforcement confidence.
- **Alternatives considered**:
  - Rely only on manual template reviews (rejected: brittle)
  - Defer all checks to `nfw check` only (rejected: delayed feedback)

## Result

All high-impact planning ambiguities for this feature are resolved.
