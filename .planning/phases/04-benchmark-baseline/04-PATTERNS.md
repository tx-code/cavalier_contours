# Phase 04 Pattern Map

**Generated:** 2026-05-12
**Scope:** Benchmark baseline planning

## Planned Files and Closest Analogs

| Planned file | Role | Closest analog | Pattern to reuse |
|--------------|------|----------------|------------------|
| `cavalier_contours/Cargo.toml` | Dev-only benchmark dependency and bench target | Existing package dependency sections | Keep production `[dependencies]` unchanged; add benchmark support only for the core crate. |
| `cavalier_contours/benches/geometry_baseline.rs` | Criterion benchmark target | `cavalier_contours/tests/test_pline_parallel_offset.rs`, `test_pline_boolean.rs`, and intersection tests | Build small reusable profile helpers, call public Rust APIs, and avoid assertions inside timed loops. |
| `.planning/phases/04-benchmark-baseline/04-BENCHMARK-MAP.md` | Historical source-to-benchmark map | `03-INVENTORY.md` | Markdown tables with source path, profile family, operation family, benchmark ID, and cost notes. |
| `.planning/phases/04-benchmark-baseline/04-BENCHMARKS.md` | Baseline/provenance and command record | `03-VERIFICATION.md` and repo testing docs | Record exact commands, environment fields, generated-output policy, and scope exclusions. |

## Source-of-Truth Patterns

### Public API Measurement

- Import from `cavalier_contours::polyline::*` and use public methods such as
  `parallel_offset`, `boolean`, `find_intersects`, `create_approx_aabb_index`,
  `area`, `extents`, `path_length`, `winding_number`, `translate_mut`, and
  `arcs_to_approx_lines`.
- Use `pline_closed!` or direct `Polyline::new()` builders consistent with
  existing tests.
- Keep profile creation outside measured loops unless the benchmark name says
  it measures creation.

### Historical Profile Mapping

- Preserve old C++ profile names in Rust benchmark IDs:
  `square`, `diamond`, `circle`, `rounded_rectangle`, `profile1`, `profile2`,
  and `pathological_profile1`.
- Represent native arc profiles and no-arcs converted variants as distinct IDs.
- Use `pathological_profile1` segment counts `10`, `25`, `50`, and `100`.

### Documentation

- Every mapped row should include old repo, commit
  `31a012947aa2e7e9474e2ec90502825afe8b99a4`, license `MIT`, and source path.
- `04-BENCHMARKS.md` should name both the smoke command and full local baseline
  command.
- Generated output under `target/criterion` is not an artifact to commit.

## Implementation Landmines

- Do not regenerate `cavalier_contours_ffi.h`.
- Do not add Clipper2 calls or oracle measurements in Phase 4.
- Do not add production dependencies for benchmark-only code.
- Do not set pass/fail performance thresholds yet; this phase records a
  baseline, not a budget.

