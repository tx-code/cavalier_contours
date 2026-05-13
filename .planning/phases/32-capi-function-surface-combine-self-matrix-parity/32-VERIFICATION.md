# Phase 32 Verification

## Scope

This file closes Phase 32 C-API function-surface combine-with-self matrix parity.

## Gate Results

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass
- `cargo test --workspace -q` - pass
- `cargo fmt --all --check` - pass
- `cargo clippy --all-targets -- -D warnings` - pass
- `git diff --check` - pass
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy

## Classification Closure

| Domain | Evidence | Classification | Decision |
|--------|----------|----------------|----------|
| Function-surface combine-with-self matrix parity | `cavalier_contours_ffi/tests/test_pline.rs`, `32-CPP-CAPI-FUNCTION-SURFACE-COMBINE-SELF-PARITY.md` | parity | Keep matrix self-boolean invariants as C-API regression baseline. |
| Output vertex and input persistence invariants | `test_pline.rs`, `32-CPP-LOGIC-ALIGNMENT-MAP.md` | parity | Keep output-vertex and no-modify assertions for union/intersect/exclude/xor self operations. |
| New core logic bug in this phase | Phase 32 evidence set | bug: none new | Function-surface bridge phase; no core algorithm edits required. |

## Requirement Closure

- `PAR-70` - complete
- `PAR-71` - complete
- `PAR-72` - complete
