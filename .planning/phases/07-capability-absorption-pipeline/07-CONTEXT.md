# Phase 07: capability-absorption-pipeline - Context

**Gathered:** 2026-05-12
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase begins actual capability absorption after the audit, fixture,
benchmark, oracle, and robustness gates are in place. It selects candidate
capabilities from recorded evidence, designs one compatible boundary, implements
the first small absorbed slice, and updates tests, examples, docs, FFI notes, or
demo UI only when the selected slice needs them. It does not port broad
algorithms wholesale, add Clipper2 as a production backend, import
triangulation, expand the demo into a product redesign, or change public API/FFI
surfaces without explicit impact notes.

</domain>

<decisions>
## Implementation Decisions

### Candidate Selection

- **D-01:** Create an explicit Phase 7 candidate matrix before implementation.
  Each candidate must trace to Phase 1 audit evidence, Phase 3 historical
  evidence, Phase 5 Clipper2 oracle evidence, or Phase 6 robustness ranking.
- **D-02:** Rank candidates by user-facing value, semantic fit with the
  line-plus-bulge arc model, available fixture/oracle evidence, implementation
  blast radius, and public API/FFI impact.
- **D-03:** Select only one first implementation slice. If no candidate can be
  made deterministic and small, document the deferral reason and promote the
  next candidate instead of forcing a broad port.

### Absorption Rules

- **D-04:** Old C++ CavalierContours remains historical reference material:
  translate behavior, tests, and algorithm intent, not implementation code.
- **D-05:** Clipper2 remains polygon-only oracle/reference material. A selected
  Clipper2-derived capability must either preserve the Rust arc-aware model or
  be explicitly documented as polygon-only and not a backend replacement.
- **D-06:** Triangulation, production Clipper2 backend work, broad generated
  corpus import, and new offset join-style families stay out of this phase.
- **D-07:** The first absorbed slice should prefer existing public patterns
  (`PlineSource`, `PlineSourceMut`, `Shape`, fixture harnesses, examples) over a
  new abstraction unless the candidate clearly requires one.

### Tests, API, FFI, and UI

- **D-08:** Every absorbed capability needs focused tests first or with the
  implementation, using property comparison when vertex ordering is brittle.
- **D-09:** Public Rust API changes require docs and example coverage. FFI
  surface changes require explicit ABI impact notes and header regeneration
  only when the ABI actually changes.
- **D-10:** Demo UI changes are allowed only when the selected capability needs
  visual validation. Otherwise keep UI untouched and rely on tests/examples.

### the agent's Discretion

The agent may choose the exact candidate matrix format, candidate IDs, scoring
weights, and first implementation slice as long as the selection is evidence
backed, narrow, and verified against the rules above.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Planning

- `.planning/PROJECT.md` - mainline Rust target, no port-first rule, Clipper2
  oracle role, UI constraint, and triangulation deferral.
- `.planning/REQUIREMENTS.md` - Phase 7 requirements `CAP-01`, `CAP-02`,
  `CAP-03`, and `DEM-01`.
- `.planning/ROADMAP.md` - Phase 7 goal, success criteria, and plan skeleton.
- `.planning/STATE.md` - current workflow position.

### Evidence Inputs

- `.planning/phases/01-absorption-contract-audit/01-AUDIT.md` - cross-codebase
  capability matrix, behavior taxonomy, candidate registry, and public surface
  comparison.
- `.planning/phases/01-absorption-contract-audit/01-PROVENANCE.md` - source
  license and acceptable-use boundaries.
- `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md` - old C++
  executable fixtures, metadata-only gaps, and deferred evidence.
- `.planning/phases/04-benchmark-baseline/04-BENCHMARK-MAP.md` - historical
  benchmark family mapping and performance evidence boundaries.
- `.planning/phases/05-clipper2-oracle-boundary/05-CLIPPER2-INVENTORY.md` -
  Clipper2 polygon-only eligibility and exclusions.
- `.planning/phases/05-clipper2-oracle-boundary/05-ORACLE-EVIDENCE.md` -
  executable and metadata-only Clipper2 oracle outcomes.
- `.planning/phases/06-robustness-gap-closure/06-ROBUSTNESS-BACKLOG.md` -
  ranked robustness and deferred candidate evidence.
- `.planning/phases/06-robustness-gap-closure/06-VERIFICATION.md` - proof that
  the robustness gate is complete before capability absorption.

### Codebase Maps

- `.planning/codebase/STACK.md` - Rust version, workspace crates, dependencies,
  UI stack, and CI environment.
- `.planning/codebase/ARCHITECTURE.md` - core/FFI/UI layering, geometry model,
  trait API, and algorithm areas.
- `.planning/codebase/INTEGRATIONS.md` - public Rust API, FFI/header rules, demo
  integration, examples, and external references.

### Code Surfaces

- `cavalier_contours/src/lib.rs` - public exports.
- `cavalier_contours/src/polyline/traits.rs` - `PlineSource` and
  `PlineSourceMut` operation surface.
- `cavalier_contours/src/polyline/internal/pline_offset.rs` - offset algorithm
  integration point.
- `cavalier_contours/src/polyline/internal/pline_boolean.rs` - boolean
  algorithm integration point.
- `cavalier_contours/src/shape_algorithms/mod.rs` - multi-polyline shape offset
  surface.
- `cavalier_contours/tests/test_utils/fixture_schema.rs` - fixture metadata,
  comparison mode, operation, and provenance structures.
- `cavalier_contours/tests/test_utils/fixture_harness.rs` - executable fixture
  runner pattern.
- `examples/` - public example surface for externally visible capabilities.
- `cavalier_contours_ffi/src/lib.rs` and `cavalier_contours_ffi.h` - FFI impact
  surfaces; update only if ABI changes.
- `cavalier_contours_ui/src/scenes/` - demo scenes; update only when visual
  validation is required.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `PlineSource` / `PlineSourceMut` already expose most geometry operations used
  by callers; new capability slices should prefer these trait surfaces.
- The Phase 2 fixture schema and harness already support offset, boolean,
  properties, provenance, comparison modes, and metadata-only records.
- Examples under `examples/` are the right lightweight documentation surface for
  new public behavior.

### Established Patterns

- The core crate forbids unsafe code; unsafe pointer and ABI handling stays in
  `cavalier_contours_ffi`.
- Tests compare geometry by properties rather than literal vertex order when
  topology is equivalent but sequence may differ.
- Oracle/reference cases are classified before execution; not-comparable cases
  stay as metadata instead of becoming forced red tests.

### Integration Points

- Capability code will likely connect through `polyline/internal/*`,
  `shape_algorithms/`, or trait methods in `polyline/traits.rs`.
- Public API changes connect to docs, examples, and possibly FFI impact notes.
- UI scenes exist for offsets, booleans, and multi-polyline shape offsets, but
  should remain unchanged unless the selected slice needs visual inspection.

</code_context>

<specifics>
## Specific Ideas

- Start `07-01` with a candidate matrix rather than implementation.
- Candidate examples to classify include Phase 6 deferred boolean thresholding,
  Clipper2 polygon case promotion, old C++ API/migration-sensitive behavior,
  cleanup/degenerate helpers, and shape/polytree-style evidence. Classification
  may reject any of these if the semantic fit or blast radius is poor.
- The first implementation should be a narrow vertical slice with tests and
  either an example/doc update or an explicit note that no external surface
  changed.

</specifics>

<deferred>
## Deferred Ideas

- Triangulation remains out of scope.
- Production Clipper2 backend remains out of scope.
- Broad parser/import automation remains a later tooling requirement.
- Productized demo redesign remains out of scope unless a future phase changes
  the demo goal.

</deferred>

---

*Phase: 07-capability-absorption-pipeline*
*Context gathered: 2026-05-12*
