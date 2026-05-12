# Phase 08 Pattern Map

## Purpose

Map Phase 8 deliverables to established repository patterns so API/FFI
readiness work stays narrow and auditable.

## Planning Artifact Patterns

| New artifact | Closest analog | Pattern to follow |
|--------------|----------------|-------------------|
| `08-COMPATIBILITY-AUDIT.md` | `07-CAPABILITY-DESIGN.md` | Explicit impact table with source evidence and decisions. |
| `08-VERIFICATION.md` | `06-VERIFICATION.md`, `07-VERIFICATION.md` | Requirement closure + command result table. |
| `MIGRATION.md` | `README.md` | User-facing operational guidance with concise steps and caveats. |

## Code and Docs Patterns

| Surface | Existing pattern | Notes |
|---------|------------------|-------|
| Rust API compatibility note | `CHANGELOG.md` "Unreleased" sections | Additive API changes should be documented as non-breaking additions. |
| Top-level user guidance | `README.md` links and feature bullets | Keep migration links visible without restructuring docs. |
| FFI compatibility | `cavalier_contours_ffi/README.md`, `cavalier_contours_ffi.h` | Regenerate header only when ABI changes exist. |

## Execution Guidance

- Prefer documenting explicit no-op decisions for unchanged FFI/header surfaces.
- Keep migration guidance practical and tied to current crate limits.
- Treat full workspace gates as the readiness boundary for this phase.
