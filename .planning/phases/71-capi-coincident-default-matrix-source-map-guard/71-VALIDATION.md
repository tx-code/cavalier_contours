---
phase: 71
slug: capi-coincident-default-matrix-source-map-guard
status: complete
nyquist_compliant: true
created: 2026-05-15
---

# Phase 71 Validation Strategy

## Core Gates

- `cargo test --workspace -q`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Task Verification Map

| Task ID | Plan | Requirement | Command | Status |
|---------|------|-------------|---------|--------|
| 71-01-01 | 71-01 | PAR-187,PAR-188 | `cargo test -p cavalier_contours_ffi --test test_pline -q` | pass |
| 71-02-01 | 71-02 | PAR-189 | `Select-String -Path .planning\phases\71-capi-coincident-default-matrix-source-map-guard\71-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Deepening Outcome","Next Alignment Targets","Decision Boundary"` | pass |
| 71-03-01 | 71-03 | PAR-187,PAR-188,PAR-189 | `cargo test --workspace -q` | pass |
| 71-03-02 | 71-03 | PAR-187,PAR-188,PAR-189 | `cargo fmt --all --check` | pass |
| 71-03-03 | 71-03 | PAR-187,PAR-188,PAR-189 | `cargo clippy --all-targets -- -D warnings` | pass |
| 71-03-04 | 71-03 | PAR-187,PAR-188,PAR-189 | `git diff --check` | pass |
| 71-03-05 | 71-03 | PAR-187,PAR-188,PAR-189 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 71-03-06 | 71-03 | PAR-187,PAR-188,PAR-189 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
