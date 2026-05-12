# Phase 07 Research: Capability Absorption Pipeline

## Research Question

What must be known to plan a safe first capability absorption slice after the
audit, fixture, benchmark, oracle, and robustness gates?

## Inputs Reviewed

- `.planning/phases/01-absorption-contract-audit/01-AUDIT.md`
- `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md`
- `.planning/phases/05-clipper2-oracle-boundary/05-CLIPPER2-INVENTORY.md`
- `.planning/phases/05-clipper2-oracle-boundary/05-ORACLE-EVIDENCE.md`
- `.planning/phases/06-robustness-gap-closure/06-ROBUSTNESS-BACKLOG.md`
- `.planning/phases/06-robustness-gap-closure/06-VERIFICATION.md`
- `.planning/codebase/STACK.md`
- `.planning/codebase/ARCHITECTURE.md`
- `.planning/codebase/INTEGRATIONS.md`

## Findings

### Selection Must Precede Implementation

Phase 7 is the first actual absorption phase, but the evidence intentionally
contains mixed classes: executable green fixtures, metadata-only gaps, oracle
records, benchmark-only references, and not-comparable sources. Planning should
start with a candidate matrix, not with code edits.

The matrix should classify each candidate as:

- `absorb-now`: small, deterministic, semantically compatible, and testable.
- `evidence-only`: valuable as tests or docs but not a production capability.
- `defer`: valid but too broad for the first slice.
- `not-comparable`: outside the Rust arc-aware model or current API scope.

### Candidate Families To Consider

- Historical old C++ offset and boolean behavior records from Phase 3.
- Clipper2 polygon-only boolean and offset oracle records from Phase 5.
- Phase 6 deferred robustness/capability-adjacent records, especially boolean
  threshold behavior and Clipper2 polygon case promotion.
- Cleanup and degenerate geometry helpers if they fit existing `PlineSource` or
  `PlineSourceMut` patterns.
- FFI or migration-sensitive behavior only when the implementation slice is
  already justified by Rust behavior; Phase 8 owns broad readiness.

### Implementation Boundary

The safest first slice should use existing surfaces:

- trait methods in `cavalier_contours/src/polyline/traits.rs`;
- implementation helpers under `cavalier_contours/src/polyline/internal/`;
- `Shape` only if the selected capability is multi-polyline area behavior;
- integration tests under `cavalier_contours/tests/`;
- examples under `examples/` when the public API changes.

Avoid adding new dependencies, unsafe code in the core crate, broad parsers, or
runtime Clipper2 linkage.

## Validation Architecture

Phase 7 validation should be staged:

1. Candidate matrix validation: prove every candidate has provenance,
   classification, semantic fit, and a decision.
2. Design validation: prove the selected slice has behavior, API/FFI/UI impact,
   and test strategy recorded before code changes.
3. Implementation validation: run targeted tests for the selected slice plus
   workspace gates.
4. External surface validation: update examples/docs/FFI notes/UI only when the
   design says they are affected.

Required commands:

- `cargo test -p cavalier_contours --test <selected_test>`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `git status --short -- target cavalier_contours/target`
- `gsd-sdk query state.validate`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Pitfalls

- Do not treat a Clipper2 oracle mismatch as automatic Rust behavior.
- Do not promote metadata-only evidence into production code without a focused
  deterministic regression.
- Do not change `cavalier_contours_ffi.h` unless the FFI ABI changes.
- Do not update the demo UI unless a selected capability needs visual
  validation.
- Do not absorb more than one implementation slice in this phase.

## RESEARCH COMPLETE
