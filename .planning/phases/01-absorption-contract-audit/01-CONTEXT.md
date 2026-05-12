# Phase 01: absorption-contract-audit - Context

**Gathered:** 2026-05-12
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase delivers the absorption contract for the Rust fork. It defines what
can be absorbed from Rust `cavalier_contours`, old C++ CavalierContours, and
Clipper2; how behavior will be compared; how license/provenance will be
recorded; and how public API/FFI surfaces should be classified for future
planning.

This phase must not import fixtures, port algorithms, or implement oracle
tooling. It produces audit and provenance artifacts that later phases consume.
</domain>

<decisions>
## Implementation Decisions

### Audit Scope Boundary

- **D-01:** Use a full evidence inventory. Phase 1 must cover API/operations,
  tests, benchmarks, examples, README/algorithm notes, known limitations, and
  FFI/C API.
- **D-02:** Organize the audit as one main cross-codebase matrix plus
  partitioned appendices by source repository.
- **D-03:** Produce two primary artifacts: `01-AUDIT.md` and
  `01-PROVENANCE.md`.
- **D-04:** Require an evidence path for every main matrix entry, or explicitly
  mark the entry `not found`, `not applicable`, or `deferred`.

### External Code Usage Rules

- **D-05:** Default to evidence-first translation. External implementations are
  reference-only; tests, fixture ideas, benchmark profiles, and behavior
  expectations may be translated with provenance.
- **D-06:** Use source-specific rules. Old C++ CavalierContours is the
  historical same-lineage reference. Clipper2 is a polygon-only oracle and
  reference source, not an implementation source.
- **D-07:** Build a candidate registry in `01-AUDIT.md` for valuable external
  tests, benchmarks, examples, or behaviors. Each candidate records source path,
  capability domain, value, risk, and suggested follow-up phase.
- **D-08:** `01-PROVENANCE.md` must record repo, commit hash, license, path,
  usage intent, and notes for external references.

### Behavior Comparability Classification

- **D-09:** `exact parity` means strict property parity, not literal vertex
  sequence parity. Normalized result count, open/closed status, orientation,
  area, path length, extents, and unexpected repeat vertices are comparison
  examples.
- **D-10:** `approximate parity` is only for arc approximation and
  tolerance-bound comparisons where the approximation/tolerance policy is
  recorded.
- **D-11:** `intentional divergence` must be decision-backed by PROJECT,
  ROADMAP, CONTEXT, or phase artifacts. Otherwise classify as a `gap` or
  `not comparable`.
- **D-12:** `not comparable` means model or scope mismatch. `gap` means missing
  or different behavior inside Rust's target scope.

### API/FFI Protection Scope

- **D-13:** Compare all public integration surfaces: Rust public API, Rust C FFI
  plus generated header, old C++ header API, old C++ C API, and Clipper2 public
  operations.
- **D-14:** Classify surfaces, but not as protection-by-default. This is a fork;
  Rust API and FFI are changeable when changes serve the absorption goal.
- **D-15:** Use freedom-oriented labels: `fork-owned/changeable`,
  `migration-sensitive`, `reference-only`, and `external-oracle`.
- **D-16:** Later API/FFI changes require an impact note that names the changed
  surface, explains why the change is worthwhile, and lists affected tests,
  examples, FFI/header, and docs.

### the agent's Discretion

No areas were delegated to the agent's discretion. The user selected concrete
options for all discussed gray areas.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Planning

- `.planning/PROJECT.md` - project goal, constraints, and fork/mainline
  decisions.
- `.planning/REQUIREMENTS.md` - Phase 1 requirement IDs and traceability.
- `.planning/ROADMAP.md` - Phase 1 goal, success criteria, and plan skeleton.
- `.planning/STATE.md` - current workflow position and continuity notes.

### Codebase Maps

- `.planning/codebase/INTEGRATIONS.md` - public Rust API, FFI, generated header,
  examples, UI, and external reference boundaries.
- `.planning/codebase/ARCHITECTURE.md` - safe Rust core boundary, polyline
  model, algorithm areas, FFI layer, and UI layer.
- `.planning/codebase/CONCERNS.md` - algorithm sensitivity, boolean scope, arc
  representation, FFI drift, Clipper2 boundary, and UI/productization concerns.

No external specs or ADRs were referenced during discussion.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `.planning/codebase/*.md`: current repository map should be used as the
  starting evidence for Rust-side audit entries.
- `cavalier_contours_ffi.h` and `cavalier_contours_ffi/src/lib.rs`: key FFI
  surfaces to include in the API/FFI comparison.
- `cavalier_contours/tests/test_utils/`: existing property comparison patterns
  inform the later fixture harness, but Phase 1 only audits.

### Established Patterns

- The core Rust crate is the fork-owned mainline and currently forbids unsafe
  code.
- FFI is isolated in the FFI crate; generated header drift is a consistency
  concern even though compatibility is not protected by default.
- Current geometry tests already prefer property-oriented comparisons for
  complex polyline results.

### Integration Points

- Phase 1 artifacts should be written under
  `.planning/phases/01-absorption-contract-audit/`.
- Later planning should consume `01-AUDIT.md` for capability and candidate
  selection, and `01-PROVENANCE.md` for license/commit/path provenance.
</code_context>

<specifics>
## Specific Ideas

- Use a full evidence inventory rather than a high-level audit.
- Treat old C++ and Clipper2 differently because old C++ is same-lineage
  historical reference while Clipper2 is a polygon-only oracle/reference.
- Do not mark existing Rust API or FFI as protected by default; this is a fork
  and these surfaces are changeable with explicit impact notes.
</specifics>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope.
</deferred>

---

*Phase: 01-absorption-contract-audit*
*Context gathered: 2026-05-12*
