# Phase 22 Verification

## Scope

This file closes Phase 22 C-API combine self-invariants parity bridge.

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
| C-API combine-with-self invariants | `test_pline.rs`, `22-CPP-CAPI-COMBINE-SELF-INVARIANTS-PARITY.md` | parity | Keep invariant bridge test as a permanent C-API correctness anchor. |
| Reversed and mixed-orientation empty-result cases | `test_pline.rs`, `22-CPP-CAPI-COMBINE-SELF-INVARIANTS-PARITY.md` | parity | Preserve explicit mixed-orientation Not/Xor empty checks. |
| New core logic bug in this phase | Phase 22 evidence set | bug: none new | Bridge-only phase; no core algorithm edits required. |

## Requirement Closure

- `PAR-40` - complete
- `PAR-41` - complete
- `PAR-42` - complete
