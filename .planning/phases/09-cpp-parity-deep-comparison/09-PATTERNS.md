# Phase 09 Pattern Map

## Purpose

Capture how C++ parity artifacts map to existing Rust test and module patterns.

| Need | Existing analog | Pattern |
|------|------------------|---------|
| C++ case promotion | `test_historical_cavalier_contours.rs` | Keep provenance and case IDs explicit. |
| Boolean property parity | `test_pline_boolean.rs` | Compare properties with tolerance and order-independent matching. |
| Mismatch reporting | `06-ROBUSTNESS-BACKLOG.md` | Classify as bug/divergence/not-comparable with evidence. |
| Final gate | `07-VERIFICATION.md`, `08-VERIFICATION.md` | Use targeted + workspace + GSD health gates. |
