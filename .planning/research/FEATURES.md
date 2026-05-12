# Feature Research

**Domain:** Arc-aware 2D computational geometry library absorption
**Researched:** 2026-05-12
**Confidence:** HIGH for repository-derived features, MEDIUM for future capability priority

## Feature Landscape

### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Cross-codebase capability inventory | Prevents vague "absorption" work | MEDIUM | Compare Rust, old C++, and Clipper2 by operation and geometry model |
| Fixture import/normalization | Needed before safe algorithm changes | HIGH | Preserve source, tolerances, expected properties, and approximation rules |
| Differential oracle harness | Makes Clipper2 useful without becoming runtime dependency | HIGH | Restrict to polygon-only or explicitly approximated cases |
| Historical C++ regression mining | Old repo contains tests, benchmarks, and algorithm explanations | MEDIUM | Prioritize offset, boolean/combine, C API, spatial index cases |
| Robustness backlog | User selected robustness as a core target | MEDIUM | Track tolerances, degenerates, repeats, tangencies, overlaps |
| Benchmark baseline | Absorption needs performance guardrails | MEDIUM | Start with old C++ profile families and current Rust cargo benches/tests |
| FFI impact tracking | C API compatibility matters for migration users | MEDIUM | Header drift and status codes need explicit gates |

### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Arc-aware parity taxonomy | Distinguishes exact, approximate, and intentionally different behavior | MEDIUM | Critical for comparing against polygon-only Clipper2 |
| Property-based geometry comparison | Avoids brittle exact vertex ordering checks | MEDIUM | Area, extents, path length, orientation, containment, no repeat vertices |
| Clipper2 case importer | Leverages active polygon test corpus | HIGH | Needs license and representation boundaries |
| C++ migration compatibility report | Helps old users move to Rust/FFI | MEDIUM | Compare API names, option defaults, status behavior |
| Visual demo validation for new features | Useful when behavior is hard to inspect textually | MEDIUM | Only when new functionality requires UI exploration |

### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Clipper2 backend mode | Appears to add robustness quickly | Not arc-aware, changes packaging and semantics | Use Clipper2 as oracle and fixture source |
| Port algorithms before tests | Feels like direct progress | High risk of silent regressions | Build audit and differential harness first |
| Triangulation parity | Clipper2 advertises triangulation | User deferred it, and Clipper2 warns about bugs | Exclude from current roadmap |
| UI productization | Demo could expose new functionality | Distracts from geometry kernel absorption | Feature-driven demo updates only |

## Feature Dependencies

```text
Capability inventory
    -> Fixture schema
        -> Historical C++ fixture import
        -> Clipper2 polygon oracle
            -> Robustness gap ranking
                -> Algorithm fixes
                    -> Capability absorption

Benchmark baseline
    -> Performance guardrails
        -> Algorithm fixes

FFI impact tracking
    -> Migration compatibility report
        -> Public API and header change gates
```

### Dependency Notes

- Inventory must precede roadmap detail because it determines what is already covered.
- Fixture schema must precede importer work so evidence is comparable over time.
- Clipper2 oracle depends on clear arc approximation and polygon-only eligibility rules.
- Algorithm fixes should follow gap ranking instead of ad hoc porting.

## MVP Definition

### Launch With

- [ ] Three-codebase audit with explicit scope and license boundaries.
- [ ] Fixture schema for Rust-native, old C++ mined, and Clipper2-derived cases.
- [ ] Initial benchmark baseline for offsets, booleans, intersections, and spatial index use.
- [ ] First robustness backlog ranked by failure risk and user-visible impact.

### Add After Validation

- [ ] Clipper2 polygon oracle runner for eligible boolean/offset cases.
- [ ] Old C++ test/benchmark importer or manual fixture translation path.
- [ ] Focused fixes for top-ranked repeat/tolerance/tangent/overlap issues.
- [ ] FFI migration and header-change checklist.

### Future Consideration

- [ ] New join styles or expanded offset options.
- [ ] Broader boolean support beyond two closed non-self-intersecting polylines.
- [ ] UI scenes for newly absorbed capabilities.
- [ ] Triangulation, only after explicit rescoping.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Codebase absorption audit | HIGH | MEDIUM | P1 |
| Fixture schema | HIGH | MEDIUM | P1 |
| Benchmark baseline | HIGH | MEDIUM | P1 |
| Clipper2 oracle harness | HIGH | HIGH | P1 |
| Old C++ fixture mining | HIGH | MEDIUM | P1 |
| Robustness fixes | HIGH | HIGH | P1 |
| Capability expansion | MEDIUM | HIGH | P2 |
| UI demo updates | MEDIUM | MEDIUM | P3 |
| Triangulation | LOW now | HIGH | Deferred |

## Competitor / Reference Analysis

| Feature | Old C++ CavalierContours | Clipper2 | Our Approach |
|---------|--------------------------|----------|--------------|
| Arc-aware polylines | Native bulge arcs | Polygon paths | Preserve Rust arc-aware model |
| Offsetting | Primary algorithm with arc support | Polygon offsetting | Compare polygon-only and mined historical cases |
| Boolean operations | Combine closed polylines | Rich polygon clipping | Use Clipper2 as polygon oracle, keep documented Rust scope explicit |
| Benchmarks | Historical profiles and Clipper comparison | C++ benchmark examples | Normalize into Rust performance guardrails |
| FFI/C API | C API header | DLL/exported functions | Keep Rust FFI as migration surface |

## Sources

- `.planning/PROJECT.md`
- `.planning/codebase/*.md`
- `E:\Coding\CavalierContours\tests\` and `README.md`
- `E:\Coding\Clipper2\CPP\Tests`, `CPP\Examples`, and `README.md`

---
*Feature research for: arc-aware geometry absorption*
*Researched: 2026-05-12*
