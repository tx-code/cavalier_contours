# Phase 05 Clipper2 Oracle Inventory

## Source Snapshot

| Field | Value |
|-------|-------|
| Source repo | `E:/Coding/Clipper2` |
| Commit | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` |
| License | `Boost Software License 1.0` |
| Usage label | `OracleComparable` / external oracle |
| Default comparison | property parity by result count, area, path length, extents, and classification notes |
| Production policy | no Clipper2 runtime/backend dependency in Phase 5 |

## Source Family Registry

| Source path | Evidence | Eligibility | Phase 5 treatment | Rationale |
|-------------|----------|-------------|-------------------|-----------|
| `Tests/Polygons.txt` | Text fixture cases with `CLIPTYPE`, `FILLRULE`, `SOL_AREA`, `SOL_COUNT`, `SUBJECTS`, and `CLIPS`. | mixed | metadata-selected candidates only | Many cases are multi-path, self-intersecting, or use broad fill-rule semantics not directly equivalent to current two-polyline Rust fixtures. |
| `CPP/Tests/TestPolygons.cpp` | Loads `Polygons.txt`, executes `Clipper64`, and compares solution area/count with case-specific tolerances. | reference policy | metadata and tolerance source | Provides expected-property policy without requiring a text parser or live Clipper2 build. |
| `CPP/Tests/TestPolytreeIntersection.cpp` | One small square intersection, no open paths, one 4-vertex result. | executable candidate | executable boolean fixture | Maps cleanly to current Rust `BooleanOp::And` over two closed area polylines. |
| `CPP/Tests/TestOffsets.cpp` | Offset cases with join type, end type, delta, and result count/orientation checks. | mixed | one executable offset fixture plus metadata records | Simple polygon collapse maps cleanly; broad offset corpus has skipped area/count or multi-path/hole behavior. |
| `Tests/Offsets.txt` | Large offset source paths with `SOL_AREA=-1` and `SOL_COUNT=-1` in loaded cases. | deferred | metadata-only | The loaded C++ loop validates orientation/exterior properties rather than stable area/count. |
| `CPP/Tests/TestOffsetOrientation.cpp` | Offset orientation checks for simple and multi-path polygons. | partial | metadata-only | Useful for later orientation evidence; multi-path order/orientation assumptions are broader than current fixture target. |
| `Tests/Lines.txt`; `CPP/Tests/TestLines.cpp` | Open path clipping behavior. | not comparable | metadata-only / deferred | Current fixture path is closed-polyline area behavior; open path clipping is out of Phase 5. |
| `Tests/PolytreeHoleOwner*.txt`; `CPP/Tests/TestPolytreeHoles.cpp`; `CPP/Tests/TestPolytreeUnion.cpp` | Hole ownership and polytree hierarchy. | not comparable | metadata-only / deferred | Current Rust boolean results have positive/negative polylines but no Clipper2 polytree ownership model. |
| `CPP/Tests/TestRectClip.cpp`; `CPP/Examples/RectClipping` | Rectangular clipping helpers. | deferred | metadata-only | Rect clipping is a separate capability, not part of the Phase 5 boolean/offset oracle minimum. |
| `CPP/Tests/TestRandomPaths.cpp`; `CPP/Examples/RandomClipping` | Randomized clipping generation and broad stress coverage. | deferred | metadata-only | Useful later if randomized differential tests are scoped; too broad for curated fixture phase. |
| `CPP/Tests/TestSimplifyPath.cpp`; `TestTrimCollinear.cpp`; `TestIsCollinear.cpp` | Cleanup, simplification, and collinearity behavior. | deferred | metadata-only | Useful for robustness ranking but not an executable boolean/offset oracle in Phase 5. |
| `CPP/Examples/SimpleClipping`; `UnionClipping`; `PolygonSamples` | Public API examples for clipping. | partial | reference-only | Useful as readable examples; selected tests provide stronger expected assertions. |
| `CPP/Examples/Inflate`; `VariableOffset` | Offset API examples, including open path end types. | partial/deferred | reference-only | Open path and variable offset behavior exceed current closed polygon fixture target. |
| `CPP/Clipper2Lib/include/clipper2/clipper.triangulation.h`; `CPP/Examples/Triangulation` | Triangulation API and samples. | excluded | deferred | Triangulation is explicitly out of scope for this roadmap slice. |
| `CPP/Clipper2Lib/include/clipper2/clipper.minkowski.h` and language equivalents | Minkowski operations. | deferred | metadata-only | Capability candidate only after gap ranking; no current fixture mapping in Phase 5. |
| `CPP/Examples/UsingZ`; export header tests | Z callback and C export surface. | not comparable | metadata-only | These are Clipper2 API/ABI concerns, not Rust geometry oracle fixtures. |

## Selected Case Registry

| Fixture ID | Source path | Operation | Expected oracle property | Comparison mode | Approximation/tolerance | Final Phase 5 status |
|------------|-------------|-----------|--------------------------|-----------------|-------------------------|----------------------|
| `clipper2-polytree-intersection-square-overlap` | `CPP/Tests/TestPolytreeIntersection.cpp` | Boolean intersection | one 4-vertex square, area `16`, path length `16`, extents `(1,1)-(5,5)` | `ApproximateParity` | default Phase 2 tolerance; compare absolute area because orientation may differ | executable |
| `clipper2-offset-007-collapsed-square` | `CPP/Tests/TestOffsets.cpp` | Offset | empty result for square `{0,0 100,0 100,100 0,100}` with `InflatePaths(..., -50, JoinType::Miter, EndType::Polygon)` | `ApproximateParity` | no arc approximation; Clipper2 negative polygon delta maps to current Rust interior offset expectation | executable |
| `clipper2-polygons-017-intersection-evenodd` | `Tests/Polygons.txt`; `CPP/Tests/TestPolygons.cpp` | Boolean intersection | `SOL_AREA=14779`, `SOL_COUNT=1` | `NotComparable` in Phase 5 | even-odd text fixture with a 6-vertex subject and clip polygon; selected for future manual verification | metadata-only selected candidate |
| `clipper2-offsets-001-round-polygon` | `Tests/Offsets.txt`; `CPP/Tests/TestOffsets.cpp` | Offset | loaded with `JoinType::Round`, `EndType::Polygon`, `Execute(1, outputs)`, stored area/count skipped | `NotComparable` in Phase 5 | lacks stable stored area/count; useful for future qualitative orientation/exterior checks | metadata-only selected candidate |
| `clipper2-open-lines-suite` | `Tests/Lines.txt`; `CPP/Tests/TestLines.cpp` | Open path clipping | open subject result behavior | `NotComparable` | current Rust fixture operation requires closed area polylines | metadata-only boundary record |
| `clipper2-triangulation-suite` | `CPP/Examples/Triangulation`; triangulation headers | Triangulation | triangle mesh output | `NotComparable` | explicitly excluded by project scope | deferred boundary record |

## Requirement Coverage

| Requirement | Coverage |
|-------------|----------|
| `FIX-04` | Clipper2 sources and selected polygon-only cases are classified with usage label, source path, comparison mode, tolerance notes, and executable/deferred treatment. |
| `ORC-01` | Selected executable boolean and offset candidates provide the input list for the dev-only comparison path. |
| `ORC-02` | Selected offset cases record join/end type, delta, and whether arc-to-polygon approximation is involved. |
| `ORC-03` | Metadata-only rows separate oracle evidence from production behavior and Phase 6 robustness decisions. |

