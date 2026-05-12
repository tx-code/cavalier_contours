# Architecture Research

**Domain:** Arc-aware 2D computational geometry library absorption
**Researched:** 2026-05-12
**Confidence:** HIGH for proposed boundaries

## Standard Architecture

### System Overview

```text
Reference Repositories
  - old C++ CavalierContours
  - Clipper2
        |
        v
Absorption Evidence Layer
  - capability inventory
  - fixture translation
  - differential oracle runners
  - benchmark baselines
        |
        v
Rust Geometry Kernel
  - core math
  - polyline model
  - intersections
  - offsets
  - booleans
  - shape offsets
        |
        v
External Surfaces
  - Rust public API
  - C FFI/header
  - examples
  - demo UI when needed
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| Capability inventory | Identify overlap, gaps, and behavior differences | Markdown/structured tables under `.planning` |
| Fixture schema | Store normalized geometry cases and expected properties | Rust test fixtures, JSON/serde where useful |
| Old C++ importer | Mine tests/benchmarks/reference cases | Manual translation first, automated later if useful |
| Clipper2 oracle | Compare polygon-only cases | Test helper or separate dev-only harness |
| Rust algorithms | Own production behavior | Existing `cavalier_contours` modules |
| FFI boundary | Expose stable C ABI | Existing `cavalier_contours_ffi` crate |
| UI demo | Visualize new functionality | Existing egui scenes, only when needed |

## Recommended Project Structure

```text
.planning/
  codebase/              # current repository map
  research/              # project-level research
  phases/                # phase plans and verification

cavalier_contours/
  src/
    core/                # math and numeric traits
    polyline/            # model, traits, algorithms
    shape_algorithms/    # multi-polyline offsets
  tests/
    test_utils/          # shared fixture/property helpers

cavalier_contours_ffi/
  src/lib.rs             # C ABI boundary
  tests/                 # FFI behavior tests

examples/                # runnable API examples
cavalier_contours_ui/    # demo-only visualization
```

### Structure Rationale

- Keep production Rust code in existing crates; do not add a C++ dependency to normal builds.
- Put absorption evidence in planning first, then promote stable cases into tests.
- Extend `tests/test_utils/` for property comparison and fixture normalization.
- Keep Clipper2 and old C++ integration in dev-only tooling unless explicitly productized.

## Architectural Patterns

### Pattern 1: Oracle-Isolated Comparison

**What:** Run reference libraries outside the production API path and compare normalized output.
**When to use:** Clipper2 polygon-only booleans/offsets or old C++ historical cases.
**Trade-offs:** More tooling, but avoids runtime dependency and semantic drift.

### Pattern 2: Property-Based Regression Fixtures

**What:** Compare geometry by properties instead of exact vertex order.
**When to use:** Offsets, booleans, shape offsets, and stitched slices.
**Trade-offs:** Must choose properties carefully so failures are meaningful.

### Pattern 3: Explicit Behavior Taxonomy

**What:** Label each reference comparison as exact parity, approximate parity, intentional divergence, or not comparable.
**When to use:** Any cross-library case.
**Trade-offs:** Requires discipline, but prevents Clipper2 from silently defining arc behavior.

## Data Flow

### Absorption Flow

```text
Reference test/benchmark/example
    -> classify geometry model
    -> translate to fixture
    -> run Rust behavior
    -> optionally run reference oracle
    -> compare normalized properties
    -> record gap or validate behavior
    -> plan fix or capability phase
```

### Algorithm Change Flow

```text
Gap report
    -> focused phase plan
    -> regression fixture
    -> implementation change
    -> workspace test gate
    -> FFI/header/doc impact check
```

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| Dozens of fixtures | Manual fixture translation is acceptable |
| Hundreds of fixtures | Add fixture manifest and grouped test runners |
| Thousands of generated cases | Add filtering, shrinking, and benchmark separation |

### Scaling Priorities

1. First bottleneck: unclear expected behavior. Fix with taxonomy and source annotations.
2. Second bottleneck: slow oracle runs. Fix with dev-only focused runners and cached fixtures.
3. Third bottleneck: brittle comparisons. Fix with property comparison helpers.

## Anti-Patterns

### Anti-Pattern 1: Production Backend Swap

**What people do:** Route operations through Clipper2 for "robustness."
**Why it is wrong:** It changes arc semantics and creates packaging/safety complexity.
**Do this instead:** Use Clipper2 as a test oracle for eligible polygon cases.

### Anti-Pattern 2: Fixture Dump Without Classification

**What people do:** Copy many cases into tests without source, tolerance, or expected behavior.
**Why it is wrong:** Failures become impossible to interpret.
**Do this instead:** Store provenance, geometry class, comparison mode, and tolerances.

### Anti-Pattern 3: UI-First Algorithm Work

**What people do:** Build visual demo changes before behavior is stable.
**Why it is wrong:** It creates surface area without durable correctness.
**Do this instead:** Add UI scenes after tests establish behavior.

## Integration Points

### External References

| Reference | Integration Pattern | Notes |
|-----------|---------------------|-------|
| old C++ CavalierContours | Manual mining, fixture translation, benchmark profile import | MIT license, historical behavior |
| Clipper2 | Dev-only oracle for polygon-only cases | Boost 1.0 license, triangulation excluded |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| core crate to FFI | Typed Rust wrappers to C ABI | Preserve status code and null handling |
| core tests to fixtures | Rust helpers/macros | Keep expected properties explicit |
| core crate to UI | Public Rust API with serde where needed | UI should not own algorithm behavior |

## Sources

- `.planning/codebase/ARCHITECTURE.md`
- `cavalier_contours/src/lib.rs`
- `cavalier_contours/tests/test_utils/`
- `E:\Coding\CavalierContours\README.md`
- `E:\Coding\Clipper2\CPP\Clipper2Lib`

---
*Architecture research for: arc-aware geometry absorption*
*Researched: 2026-05-12*
