# Phase 07 Capability Candidates

## Ranking Method

Score is 1-5 for user-facing value, semantic fit, evidence strength, low blast
radius, and low public-surface risk. Higher total is better for the first Phase
7 slice.

## Candidate Matrix

| Rank | Candidate ID | Source evidence | Source family | Semantic fit | User value | Evidence | Low blast | API/FFI risk | Total | Classification | Decision |
|------|--------------|-----------------|---------------|--------------|------------|----------|-----------|--------------|-------|----------------|----------|
| 1 | `rect-clip-convenience` | Phase 1 `01-AUDIT.md` RectClip source appendix; Phase 5 `05-CLIPPER2-INVENTORY.md` rect clipping deferred record | `clipper2` | 5 | 4 | 4 | 5 | 4 | 22 | `absorb-now` | **Selected first slice**. Add a small Rust convenience API that clips a closed polyline to an axis-aligned rectangle by reusing existing arc-aware boolean intersection. No Clipper2 runtime dependency. |
| 2 | `boolean-collapsed-area-thresholding` | Phase 6 `06-ROBUSTNESS-BACKLOG.md` rank 2 current Rust gap | `current-rust-gap` | 4 | 4 | 3 | 3 | 3 | 17 | `defer` | Valuable but needs a public-case failing regression before changing defaults or ergonomics. |
| 3 | `clipper2-polygons-017-intersection-evenodd` | Phase 5 `05-ORACLE-EVIDENCE.md`; Phase 6 backlog rank 3 | `clipper2` | 3 | 4 | 3 | 3 | 4 | 17 | `evidence-only` | Keep as future oracle promotion; mapping from text fixture to two-polyline Rust boolean is not yet precise enough for first slice. |
| 4 | `historical-cpp-combine-circle-rectangle-union` | Phase 3 `03-INVENTORY.md`; Phase 6 backlog rank 4 | `old-cpp` | 3 | 2 | 5 | 5 | 5 | 20 | `evidence-only` | Executable geometry parity is already green; remaining vertex-count delta is a representation/topology detail only. |
| 5 | `offset-round-orientation-exterior-corpus` | Phase 5 `05-ORACLE-EVIDENCE.md`; Phase 6 backlog rank 5 | `clipper2` | 3 | 3 | 3 | 2 | 4 | 15 | `defer` | Needs stronger expected properties before execution; avoid broad offset semantics changes in first slice. |
| 6 | `open-path-clipper-lines-suite` | Phase 5 `05-ORACLE-EVIDENCE.md`; Phase 6 backlog rank 6 | `clipper2` | 1 | 3 | 4 | 1 | 3 | 12 | `not-comparable` | Open-path clipping does not match current closed area boolean scope. |
| 7 | `spatial-index-query-behavior-record` | Phase 3 `03-INVENTORY.md`; Phase 4 benchmark mapping | `old-cpp` | 4 | 2 | 5 | 5 | 5 | 21 | `evidence-only` | Query/visit semantics already have executable parity coverage; keep this candidate as benchmark/reference evidence rather than public capability absorption. |

## Selected First Slice

`rect-clip-convenience` is the selected first slice for Phase 7.

The absorbed behavior is a small convenience capability inspired by Clipper2's
RectClip family, but implemented through existing Rust `BooleanOp::And`
semantics against a generated axis-aligned rectangle. This keeps Rust
`cavalier_contours` as the mainline implementation, preserves native bulge-arc
handling where current boolean behavior supports it, and avoids any Clipper2
production dependency.

Expected impact:

- Public Rust API: add a small `PlineSource` convenience method pair if design
  confirms the surface.
- FFI: none for this phase.
- Generated header: none.
- UI: none unless design discovers visual validation is required.
- Tests/examples: required, because this is externally visible Rust behavior.

## Deferred / Rejected Candidates

- `boolean-collapsed-area-thresholding`: defer until a focused public
  regression proves a default or API change is needed.
- `clipper2-polygons-017-intersection-evenodd`: evidence-only until the manual
  mapping is precise enough to avoid parser work.
- `historical-cpp-combine-circle-rectangle-union`: evidence-only because
  geometry parity is already executable-green; the remaining difference is
  vertex-count representation, not a proven geometry property failure.
- `offset-round-orientation-exterior-corpus`: defer because qualitative
  orientation/exterior checks are not yet a narrow implementation slice.
- `open-path-clipper-lines-suite`: not-comparable with the current closed-area
  boolean scope.
- `spatial-index-query-behavior-record`: evidence-only; Phase 4 already
  captures the benchmark/reference value.
- triangulation: deferred by project scope and Clipper2 warnings.
- production Clipper2 backend: out of scope; Clipper2 remains an oracle and
  reference.
- broad parser/import automation: later tooling work, not the first absorption
  slice.
- UI redesign: out of scope; UI changes must stay tied to required visual
  validation.

## Requirement Coverage

| Requirement | Coverage |
|-------------|----------|
| `CAP-01` | Candidate capabilities are ranked from Phase 1 audit, Phase 3 historical evidence, Phase 5 oracle evidence, and Phase 6 robustness backlog rather than port-first intuition. |
