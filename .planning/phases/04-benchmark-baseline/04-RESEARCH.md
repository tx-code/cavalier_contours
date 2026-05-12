# Phase 04: benchmark-baseline - Research

**Researched:** 2026-05-12
**Domain:** Rust benchmark baseline and historical C++ benchmark mapping
**Confidence:** HIGH

<user_constraints>
## User Constraints from CONTEXT.md

- Use a stable Rust benchmark harness, preferably Criterion.
- Put benchmark definitions under `cavalier_contours/benches/`.
- Do not commit generated benchmark outputs such as `target/criterion`.
- Cover offsets, booleans, intersections, and spatial-index-heavy inputs.
- Map old C++ profile families: square, diamond, circle, rounded rectangle,
  profile1, profile2, and pathologicalProfile1.
- Keep native arc-aware and arc-to-line converted cases separate. Conversion
  cost is excluded unless a benchmark explicitly measures conversion.
- Keep Clipper2 runtime and oracle costs out of Phase 4.
- Add dev-only benchmark dependencies only; do not change production geometry
  behavior, FFI headers, or UI.
</user_constraints>

<research_summary>
## Summary

Phase 4 should introduce a single Criterion benchmark target first, then expand
it with profile families translated from old C++ benchmark inputs. The target
should measure current Rust APIs through public library calls: `parallel_offset`,
`boolean`, `find_intersects`, `create_approx_aabb_index`, `area`, `extents`,
`path_length`, and `winding_number`.

The old C++ benchmark suite is a profile and workload source, not an
implementation source. Its useful contract is the shape family, operation loop,
arc-to-line conversion mode, and setup boundary. Generated Criterion output must
stay untracked; committed docs should record benchmark IDs, commands,
environment fields, and cost-accounting rules.

**Primary recommendation:** add `criterion` as a dev-dependency for the core
crate, create `cavalier_contours/benches/geometry_baseline.rs`, document the
mapping in `04-BENCHMARK-MAP.md`, and keep final baseline/provenance notes in
`04-BENCHMARKS.md`.
</research_summary>

<standard_stack>
## Standard Stack

| Tool | Scope | Purpose |
|------|-------|---------|
| Criterion | dev-dependency only | Stable benchmark harness and `cargo bench` integration. |
| `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` | smoke gate | Compile and execute the benchmark binary in a practical validation mode. |
| `cargo bench -p cavalier_contours --bench geometry_baseline` | local baseline | Run measurement locally and write generated output under `target/`. |
| Rust workspace gates | final validation | Keep normal build, test, format, and lint contracts green. |

No production dependency is needed. The existing `static_aabb2d_index`
dependency is already part of the core crate and should be used through current
Rust APIs.
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Benchmark Target

Use one benchmark target unless implementation proves it is too large:

```text
cavalier_contours/benches/
  geometry_baseline.rs
```

The file can contain local profile builders at first. Split to helper modules
only if the file becomes difficult to navigate.

### Workload Groups

Recommended Criterion group names:

- `offset/native/*` and `offset/no_arcs/*`
- `boolean/shifted/*` and `boolean/coincident/*`
- `intersections/current/*`
- `spatial_index/create/*` and `spatial_index/query_reuse_stack/*`
- `properties/area/*`, `properties/extents/*`, `properties/path_length/*`,
  and `properties/winding_number_grid/*`

Use `criterion::black_box` around inputs and outputs. Use setup outside the
timed loop when the old C++ source had a setup type, such as shifted boolean
profiles, spatial index query state, and winding-number point grids.

### Historical Source Mapping

Use old C++ source paths as provenance:

- `tests/benchmarks/benchmarkprofiles.h`
- `tests/benchmarks/offsetbenchmarks.cpp`
- `tests/benchmarks/combinebenchmarks.cpp`
- `tests/benchmarks/spatialindexbenchmarks.cpp`
- `tests/benchmarks/areabenchmarks.cpp`
- `tests/benchmarks/extentsbenchmarks.cpp`
- `tests/benchmarks/pathlengthbenchmarks.cpp`
- `tests/benchmarks/windingnumberbenchmarks.cpp`

The old suite runs native arc profiles and converted no-arcs variants with
`arcsToLinesError = 0.01`. In Rust, build the native profile first and derive
the no-arcs profile via `arcs_to_approx_lines(0.01)` before measurement so
conversion cost is excluded.
</architecture_patterns>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Measuring Setup as Operation Cost

If shifted copies, query stacks, or no-arcs conversion are built inside the
timed loop, benchmark numbers cannot be compared to the old profile intent.
Keep setup outside `bench_function` iteration unless the benchmark ID names that
setup explicitly.

### Pitfall 2: Redefining Phase 4 as Performance Budgeting

Phase 4 creates baseline coverage and provenance. It should not add thresholds,
CI regression budgets, or optimization work.

### Pitfall 3: Mixing Clipper2 Costs Into the Baseline

Clipper2 oracle/runtime policy belongs to Phase 5. Phase 4 docs may mention
that oracle cost is excluded, but benchmark code should not call Clipper2.

### Pitfall 4: Benchmark Code Touching Production Behavior

All source changes should stay in `Cargo.toml` dev/bench declarations and
`cavalier_contours/benches/`. Any change under `cavalier_contours/src/`,
`cavalier_contours_ffi/`, or `cavalier_contours_ui/` is scope drift unless a
later phase explicitly opens that boundary.
</common_pitfalls>

<validation_architecture>
## Validation Architecture

### Automated Checks

- `cargo bench -p cavalier_contours --bench geometry_baseline -- --test`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`

### Documentation Checks

- `04-BENCHMARK-MAP.md` names all required old C++ profile families and source
  files.
- `04-BENCHMARKS.md` states environment fields, commands, harness mode,
  generated-output policy, and included/excluded setup, conversion, and oracle
  costs.
- `git status --short -- target` should show no committed generated benchmark
  output.
</validation_architecture>

## RESEARCH COMPLETE

