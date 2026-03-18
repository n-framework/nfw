# Research: Template Catalog Listing and Selection

## Decision: Use a stable template identifier as the public selection key and keep display-friendly metadata separate

**Rationale**: The current template model exposes one `Name` value plus a description. This is not sufficient for both a long-lived machine-typed selector and a readable prompt/listing label. Treating the identifier as the stable public contract and adding separate display metadata keeps non-interactive usage reliable while allowing interactive output to stay human-friendly.

**Alternatives considered**:

- Reuse a single human-readable name for both display and selection: rejected because renaming display text would break automation.
- Introduce numeric identifiers: rejected because they are opaque to users and difficult to maintain across catalog revisions.

## Decision: Preserve the validated catalog sequence for both listing and interactive prompts

**Rationale**: One shared sequence across `nfw templates` and `nfw new` avoids user confusion and lets catalog authors curate the visible order. Determinism comes from validating and preserving the same ordered catalog content within a given catalog version.

**Alternatives considered**:

- Sort alphabetically by identifier at runtime: rejected because it overrides curated order and can separate related templates.
- Add a second ordering mechanism independent of the catalog sequence: rejected because it adds avoidable complexity for an initial feature slice.

## Decision: Treat a session as interactive only when terminal input and output are both attached to a TTY

**Rationale**: Prompting requires the user to see choices and send a response. Requiring both sides of the interaction to be connected to a terminal avoids hangs or corrupted output in CI, redirected streams, and scripted execution.

**Alternatives considered**:

- Check only output interactivity: rejected because redirected or piped input can still make prompting unsafe.
- Always prompt when `--template` is absent: rejected because automation would fail unpredictably.

## Decision: Missing or unknown template identifiers are usage errors and must fail before workspace generation starts

**Rationale**: These failures are caused by incomplete or invalid user input, not by runtime instability. Treating them as usage errors keeps the contract actionable and ensures the command never creates partial output before the user has made a valid selection.

**Alternatives considered**:

- Implicitly choose a default template: rejected because it hides selection decisions and breaks deterministic automation.
- Treat invalid identifiers as runtime failures: rejected because it weakens error semantics and makes diagnostics less precise.

## Decision: Unit tests must use fixture catalogs and fake selection/interactivity collaborators

**Rationale**: The feature is driven by catalog parsing, deterministic ordering, and terminal-mode branching. These are all testable without network calls or real terminal sessions. Using fixtures and fakes satisfies the constitution requirement for deterministic tests and keeps failures local and fast.

**Alternatives considered**:

- Hit the live release catalog in tests: rejected because network dependency would make tests flaky and slow.
- Rely only on end-to-end console tests: rejected because it would make failure diagnosis harder and leave parsing/validation edge cases under-tested.
