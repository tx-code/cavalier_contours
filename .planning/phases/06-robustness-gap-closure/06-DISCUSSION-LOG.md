# Phase 06 Discussion Log

## User Direction

- Continue the GSD flow with fewer questions.
- Use the evidence pipeline already built in Phases 1-5.
- Avoid unclear, broad algorithm absorption before ranking and verification.

## Assumptions

- Phase 6 should make at least one narrow production robustness improvement if
  the focused regression confirms a current gap.
- No user checkpoint is needed before choosing the first high-confidence fix
  target because the earlier phase goals and constraints are clear.
- Public Rust APIs, FFI, generated headers, UI, and benchmark baselines stay
  unchanged unless the focused fix proves otherwise.

## Chosen Direction

Use `Shape::from_plines` degenerate/repeat input handling as the first fix
target because it is narrow, evidence-aligned, and adjacent to already proven
polyline offset repeat-position robustness work.

