# Phase 07: capability-absorption-pipeline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-12
**Phase:** 07-capability-absorption-pipeline
**Areas discussed:** Candidate selection, absorption boundaries, API/FFI/UI impact

---

## Candidate Selection

| Option | Description | Selected |
|--------|-------------|----------|
| Preselect a capability during context capture | Faster start, but risks port-first bias before reviewing Phase 1-6 evidence. | |
| Build an evidence-ranked candidate matrix first | Uses existing audit, fixture, oracle, benchmark, and robustness artifacts before selecting a slice. | yes |
| Absorb multiple capabilities at once | Higher throughput, but higher review and regression risk. | |

**User's choice:** Inferred from prior instruction to keep phases evidence-led and ask fewer questions.
**Notes:** Phase 7 context locks the matrix-first approach and limits implementation to one initial slice.

---

## Absorption Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Port old C++ or Clipper2 algorithms directly | Broad implementation movement with weak fit for the Rust arc-aware model. | |
| Translate behavior and tests, then implement narrowly | Preserves Rust ownership and keeps external sources as references/oracles. | yes |
| Add Clipper2 as a production backend | Explicitly out of scope from project decisions. | |

**User's choice:** Carry forward prior project decisions.
**Notes:** Triangulation, production Clipper2 backend work, broad corpus import, and unrelated UI redesign remain deferred.

---

## API/FFI/UI Impact

| Option | Description | Selected |
|--------|-------------|----------|
| Change public API/FFI freely | Possible in a fork, but risks untracked external surface drift. | |
| Require impact notes and tests for visible changes | Matches earlier audit and AGENTS.md rules. | yes |
| Redesign the demo during absorption | Out of scope unless visual validation is required. | |

**User's choice:** Carry forward prior decision that UI changes are feature-driven.
**Notes:** `cavalier_contours_ffi.h` should only be regenerated if the ABI surface changes.

---

## the agent's Discretion

- Exact candidate matrix format, candidate IDs, scoring weights, and first slice
  selection are delegated to planning, provided evidence and scope constraints
  are preserved.

## Deferred Ideas

- Triangulation.
- Production Clipper2 backend.
- Broad parser/import automation.
- Productized demo redesign.
