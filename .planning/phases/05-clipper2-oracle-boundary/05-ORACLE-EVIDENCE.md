# Phase 05 Clipper2 Oracle Evidence

## Report Command

```powershell
$env:CAVC_CLIPPER2_ORACLE_REPORT = '1'
cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture
Remove-Item Env:CAVC_CLIPPER2_ORACLE_REPORT
```

Generated local report:

```text
target/clipper2-oracle/clipper2-oracle-report.md
```

The generated report is a local artifact and is not committed. It states that
oracle results are Phase 6 gap-ranking evidence, not production behavior.

## Selected Oracle Results

| Fixture ID | Operation | Source path | Comparison | Status | Notes |
|------------|-----------|-------------|------------|--------|-------|
| `clipper2-polytree-intersection-square-overlap` | Boolean | `CPP/Tests/TestPolytreeIntersection.cpp` | `ApproximateParity` | pass | One square/square intersection mapped to current Rust `BooleanOp::And`; compares result properties by absolute area. |
| `clipper2-offset-007-collapsed-square` | Offset | `CPP/Tests/TestOffsets.cpp` | `ApproximateParity` | pass | Clipper2 `InflatePaths(..., -50, JoinType::Miter, EndType::Polygon)` maps to current Rust interior collapse expectation for the same straight-segment square. |
| `clipper2-polygons-017-intersection-evenodd` | Boolean | `Tests/Polygons.txt`; `CPP/Tests/TestPolygons.cpp` | `NotComparable` | deferred | Kept as a selected text-fixture candidate until a precise two-polyline Rust mapping is manually verified. |
| `clipper2-offsets-001-round-polygon` | Offset | `Tests/Offsets.txt`; `CPP/Tests/TestOffsets.cpp` | `NotComparable` | deferred | The C++ test skips stored area/count and validates broader orientation/exterior behavior. |
| `clipper2-open-lines-suite` | Boolean | `Tests/Lines.txt`; `CPP/Tests/TestLines.cpp` | `NotComparable` | not comparable | Open-path clipping is outside the current closed area polyline fixture path. |
| `clipper2-triangulation-suite` | Properties | `CPP/Examples/Triangulation`; triangulation headers | `NotComparable` | deferred | Triangulation remains excluded by roadmap scope. |

## Approximation Notes

- The two executable fixtures use straight polygon segments only.
- No arc-to-polygon approximation is involved in the executable Phase 5 oracle
  set.
- Future arc-derived oracle comparisons must record the approximation tolerance
  and must not redefine native bulge-arc semantics.

## Phase 6 Handoff

- Use the passing executable oracle fixtures as a sanity baseline for
  polygon-only comparison plumbing.
- Treat deferred text fixtures as candidate evidence when ranking robustness
  gaps, especially small `Polygons.txt` boolean cases and qualitative offset
  orientation/exterior cases.
- Do not infer production Clipper2 parity from this phase; it only establishes
  the oracle boundary and a dev-only report path.

## Requirement Coverage

| Requirement | Coverage |
|-------------|----------|
| `FIX-04` | Eligible Clipper2 polygon-only cases are represented as Rust fixtures with explicit comparability classification. |
| `ORC-01` | `test_clipper2_oracle_fixtures.rs` provides a dev-only comparison/report path for selected boolean and offset cases. |
| `ORC-02` | Offset fixture notes record join/end type, delta mapping, and no arc approximation for the executable case. |
| `ORC-03` | Report and evidence docs state oracle output is gap-ranking evidence, not production behavior. |

