# Pitfalls Research

**Domain:** Arc-aware 2D computational geometry library absorption
**Researched:** 2026-05-12
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: Treating Clipper2 as Arc-Aware

**What goes wrong:**
Polygon-only results are used to judge or replace arc-aware behavior.

**Why it happens:**
Clipper2 is active and robust, so it is tempting to treat it as a universal oracle.

**How to avoid:**
Only compare Clipper2 against polygon-only cases or cases with documented arc
approximation. Mark arc-specific cases as old-C++/Rust-only unless an explicit
adaptation exists.

**Warning signs:**
Tests convert bulges to points without recording approximation tolerance.

**Phase to address:**
Audit and fixture-schema phases.

---

### Pitfall 2: Porting Before Evidence

**What goes wrong:**
Algorithm changes land without proving what behavior they preserve or improve.

**Why it happens:**
Absorption sounds implementation-heavy, but the real risk is behavioral drift.

**How to avoid:**
Require each algorithm phase to begin with fixtures, expected properties,
comparison mode, and verification commands.

**Warning signs:**
A phase says "port X" but has no failing case or benchmark target.

**Phase to address:**
Roadmap and every algorithm phase.

---

### Pitfall 3: Epsilon Drift

**What goes wrong:**
Small tolerance changes fix one case and break nearby tangent, overlap, repeat,
or degenerate cases.

**Why it happens:**
The current algorithms rely on absolute fuzzy comparisons around values such as
`1e-5` and `1e-4`, and old C++ documentation notes scale sensitivity.

**How to avoid:**
Add targeted regression fixtures and document the tolerance mode per comparison.

**Warning signs:**
Tests pass only after widening tolerances globally.

**Phase to address:**
Robustness baseline and each numeric fix phase.

---

### Pitfall 4: FFI/Header Drift

**What goes wrong:**
Rust FFI signatures or structs change but the committed header and tests do not.

**Why it happens:**
The Rust crate is easier to evolve than the generated C ABI surface.

**How to avoid:**
Any FFI-impacting phase must update FFI tests, regenerate the header, and record
API impact.

**Warning signs:**
`cavalier_contours_ffi/src/lib.rs` changes without `cavalier_contours_ffi.h`.

**Phase to address:**
FFI compatibility and any public API phase.

---

### Pitfall 5: Benchmark Theater

**What goes wrong:**
Benchmarks exist but do not represent the target failure modes or compare
against historical profiles.

**Why it happens:**
It is easy to time happy paths while ignoring pathological spatial-index and arc
cases.

**How to avoid:**
Start with old C++ benchmark profiles and add Rust baselines for square, circle,
rounded rectangle, profile, and pathological profile families.

**Warning signs:**
Performance claims are made without profile names or fixture provenance.

**Phase to address:**
Benchmark baseline phase.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Exact vertex equality for offsets | Simple assertions | False failures from equivalent geometry | Only for tiny deterministic primitive tests |
| Unclassified fixture imports | Fast case growth | Hard-to-debug failures | Never for long-lived regression tests |
| Global tolerance widening | Quick pass | Masks real geometry defects | Only as a temporary debug note, not committed policy |
| UI-only validation | Easy visual confidence | No durable gate | As supplementary evidence only |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| old C++ tests | Copy expected vertices blindly | Translate to property expectations with provenance |
| Clipper2 | Compare arc inputs directly | Compare polygon-only or explicitly approximated inputs |
| FFI | Validate only Rust APIs | Add ABI-level tests and header checks |
| Benchmarks | Mix conversion cost inconsistently | Define whether conversion/approximation is included |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Pathological overlapping AABBs | Offset/intersection operations become quadratic-like | Include old pathological profiles | High-overlap arc/profile inputs |
| Arc approximation explosion | Vertex counts grow by orders of magnitude | Preserve native arcs where possible | Circles and rounded profiles |
| Oracle in normal test gate | CI becomes slow or toolchain-dependent | Keep heavy oracle tests focused or opt-in | Large fixture corpus |

## Security / Safety Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Unsafe in core crate | Violates current safety contract | Keep unsafe isolated to FFI |
| Raw pointer dereference before checks | Undefined behavior in C ABI | Preserve null/bounds checks |
| Panic across FFI | ABI unsafety | Keep `ffi_catch_unwind!` around exports |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| API behavior changes without examples | Users cannot migrate confidently | Update examples and docs with behavior notes |
| Demo lags new feature | Hard to inspect geometry behavior visually | Add targeted scene only for new visible capability |
| Error status undocumented | FFI users guess failure causes | Keep status codes stable and documented |

## Looks Done But Is Not Checklist

- [ ] Audit: includes licenses, geometry model, API, tests, benchmarks, and exclusions.
- [ ] Fixture: records source, tolerance, comparison mode, and expected properties.
- [ ] Clipper2 oracle: excludes triangulation and non-polygon arc semantics.
- [ ] Algorithm fix: adds regression tests before or with the implementation.
- [ ] FFI change: updates tests and generated header.
- [ ] UI change: has a feature reason, not just a redesign impulse.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Clipper2 semantics leaked into arc behavior | HIGH | Reclassify fixtures, revert invalid expectations, add arc-specific tests |
| Port without evidence | MEDIUM | Freeze implementation, add fixtures, compare old/new behavior |
| Header drift | MEDIUM | Regenerate header, add ABI test, document impact |
| Benchmark mismatch | LOW | Rename benchmark, add provenance, split conversion cost |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Clipper2 treated as arc-aware | Audit / fixture schema | Every Clipper2 case has eligibility classification |
| Porting before evidence | Roadmap / test base | Each algorithm phase starts from cases or measured gaps |
| Epsilon drift | Robustness baseline | Tolerance cases cover repeats, tangencies, overlaps |
| FFI drift | FFI compatibility phase | Header and FFI tests checked together |
| Benchmark theater | Benchmark baseline | Profiles and conversion rules documented |

## Sources

- Current Rust tests and codebase map
- Old C++ README implementation notes and benchmark profiles
- Clipper2 README and local C++ tests/examples

---
*Pitfalls research for: arc-aware geometry absorption*
*Researched: 2026-05-12*
