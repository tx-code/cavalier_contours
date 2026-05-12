# Phase 09-02: C++ Offset and Intersection Parity Report

## Scope

This report expands parity evidence from boolean-only into:

- C++ offset cases from `tests/tests/TEST_cavc_parallel_offset.cpp`
- C++ intersection surface mapping from `include/cavc/polylineintersects.hpp`

## C++ to Rust Module Map

| C++ source | Rust source | Notes |
|------------|-------------|-------|
| `include/cavc/polylineoffset.hpp` | `cavalier_contours/src/polyline/internal/pline_offset.rs` | Core parallel offset behavior. |
| `tests/tests/TEST_cavc_parallel_offset.cpp` | `cavalier_contours/tests/test_cpp_offset_parity.rs` | Direct executable offset parity cases. |
| `include/cavc/polylineintersects.hpp` | `cavalier_contours/src/polyline/internal/pline_intersects.rs` | Polyline intersection collection and filtering. |
| segment/arc intersection helpers | `cavalier_contours/src/polyline/pline_seg_intersect.rs` | Primitive segment intersection behavior. |

## Executed Offset Outcomes

Evidence command:
`cargo test -p cavalier_contours --test test_cpp_offset_parity -- --nocapture`

| C++ case | Result | Classification |
|----------|--------|----------------|
| `closed_rectangle_inward` | parity pass (`vertex_count=4`, `area=96`, `path_length=44`, extents `(2,2)-(18,8)`) | parity |
| `closed_rectangle_outward` | parity pass (`vertex_count=8`, `area=332.56637061436`, `path_length=72.566370614359`, extents `(-2,-2)-(22,12)`) | parity |
| `collapsed_rectangle` | parity pass (empty result) | parity |

## Intersection Evidence and Classification

`test_cpp_offset_parity.rs` also records a deterministic intersection snapshot
for the C++ circle/rectangle combine geometry (`basic_intersects=4`,
`overlapping_intersects=0`) and currently matches Rust behavior.

- `bug`: none confirmed in 09-02.
- `intentional-divergence`: none confirmed in 09-02.
- `not-comparable`: no standalone old C++ intersection expectation table is
  currently available in this repository snapshot; direct one-to-one
  case-to-expected intersection parity remains partially not-comparable.

## 09-02 Decision

Offset parity is currently green on selected high-value C++ cases. Keep the
intersection gap explicitly classified as partially not-comparable and close it
in 09-03 with either additional old-C++ expectations or explicit defer notes.
