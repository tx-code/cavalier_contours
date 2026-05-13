# Phase 74: C-API Logic Alignment Map

This map captures next steps after aabbindex extents source parity hardening.

## Deepening Outcome

- C-API aabbindex extents parity now executes source-backed extents cases aligned
  to old `StaticSpatialIndexTests.index` and `skip_sorting_small_index`.
- Extents checks now cover both `create_approx_aabbindex` and `create_aabbindex`
  constructors and compare their extents surfaces.
- Extents hardening now includes explicit null-path and empty-index NaN behavior
  checks.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Expand source-backed aabbindex parity beyond extents (where C-API exposes corresponding behavior) | `cavalier_contours_ffi/tests/test_pline.rs` | Add only checks with deterministic source or API-doc provenance. |
| P1 | Keep extents source-case coverage diagnostics stable during future aabbindex parity growth | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve case-count/missing/duplicate guard semantics and naming. |
| P2 | Instantiate drift triage template when first real source update mismatch appears | `.planning/phases/*` | Run drift report workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_staticspatialindex.cpp`
