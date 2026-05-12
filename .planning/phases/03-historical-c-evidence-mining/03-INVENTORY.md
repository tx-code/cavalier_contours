# Phase 03 Historical C++ Evidence Inventory

## Source Snapshot

| Field | Value |
|-------|-------|
| Source repo | `E:/Coding/CavalierContours` |
| Commit | `31a012947aa2e7e9474e2ec90502825afe8b99a4` |
| License | `MIT` |
| Default tolerance | Phase 2 `FixtureTolerance::default()` (`property_eps=1e-4`, `position_eps=1e-5`) unless a row records an override. |
| Comparison policy | Prefer `PlineProperties` parity over literal vertex order. |

## Candidate Source Registry

| Source path | Evidence | Initial classification | Usage label | Notes |
|-------------|----------|------------------------|-------------|-------|
| `tests/tests/TEST_cavc_parallel_offset.cpp` | Offset result counts and properties for rectangles, diamonds, arcs, collapse cases, and reversed inputs. | executable candidate | `translated-fixture-candidate` | Use property parity; reversed area sign cases are useful later but not required for this phase. |
| `tests/tests/TEST_cavc_combine_plines.cpp` | Old `combine_mode` 0..3 boolean results with unordered property comparison and area sign ignored. | executable candidate | `translated-fixture-candidate` | Prefer explicitly constructed circle/rectangle cases; avoid broad constructor-table import. |
| `tests/tests/TEST_cavc_pline_function.cpp` | Area, path length, extents, winding, closest point, and offset expectations for circle/half-circle cases. | executable candidate | `translated-fixture-candidate` | Pure property cases need a narrow test-only `Properties` fixture operation. |
| `tests/tests/TEST_staticspatialindex.cpp` | Static spatial index add/finish/query/visit behavior, including early-stop query visitor. | metadata-only / deferred | `translated-fixture-candidate`, `benchmark-candidate` | Behavior notes only in Phase 3; performance and benchmark mapping defer to Phase 4. |
| `c_api_include/cavaliercontours.h` | Old C API surface for pline creation, lists, offset, combine, area/path/extents, winding, and closest point. | metadata-only / not comparable | `migration-sensitive` | No Rust FFI implementation or `cavalier_contours_ffi.h` change in Phase 3. |
| `src/cavaliercontours.cpp` | Old C API implementation routing to C++ polyline operations. | metadata-only / not comparable | `migration-sensitive` | Records migration-sensitive behavior shape without copying implementation code. |
| `examples/*.cpp`, `README.md` | Usage examples and behavior notes. | deferred | `reference-only`, `translated-fixture-candidate` | Inventory-only unless a later phase promotes a case. |
| `tests/benchmarks/*` | Historical benchmark profile names and cost-sensitive cases. | deferred | `benchmark-candidate` | Phase 4 owns benchmark baselines and performance treatment. |

## Selected Executable Fixture Candidates

| Fixture ID | Source path | Old C++ case | Operation kind | Comparison mode | Expected property source | Tolerance policy | Fallback if execution fails | Final status |
|------------|-------------|--------------|----------------|-----------------|--------------------------|------------------|-----------------------------|--------------|
| `historical-cpp-offset-closed-rectangle-inward` | `tests/tests/TEST_cavc_parallel_offset.cpp` | `closed_rectangle_inward` | Offset | `ApproximateParity` | `vertex_count=4`, `area=96`, `path_length=44`, extents `(2,2)-(18,8)` | default | convert to metadata-only `Gap` with observed mismatch | `executable-green` |
| `historical-cpp-offset-collapsed-rectangle` | `tests/tests/TEST_cavc_parallel_offset.cpp` | `collapsed_rectangle` | Offset | `ApproximateParity` | empty offset result | default | metadata-only `Gap` if current Rust emits geometry | `executable-green` |
| `historical-cpp-combine-circle-rectangle-union` | `tests/tests/TEST_cavc_combine_plines.cpp` | `circle_rectangle_union`, `combine_mode=0` | Boolean | `Gap` | old `expectedRemaining` row has `vertex_count=10`; current Rust produces equivalent area/path/extents with `vertex_count=8` | default | metadata-only gap record | `metadata-only-gap` |
| `historical-cpp-properties-ccw-circle-x-aligned` | `tests/tests/TEST_cavc_pline_function.cpp` | `ccw_circle_x_aligned`, radius `5`, center `(1,1)` | Properties | `ApproximateParity` | computed old test expectations: `PI*25`, `2*PI*5`, extents `(-4,-4)-(6,6)` | default | metadata-only `Gap` if current property calculation differs | `executable-green` |

## Metadata-Only Evidence

| Record ID | Source path | Usage label | Comparison mode | Operation kind | Final status | Rationale |
|-----------|-------------|-------------|-----------------|----------------|--------------|-----------|
| `historical-cpp-c-api-surface-migration-record` | `c_api_include/cavaliercontours.h`; `src/cavaliercontours.cpp` | `migration-sensitive` | `NotComparable` | Properties metadata | `metadata-only-not-comparable` | Records old construction/list/offset/combine/property function surface for migration notes. Phase 3 makes no C FFI code or generated header change. |
| `historical-cpp-static-spatial-index-query-record` | `tests/tests/TEST_staticspatialindex.cpp` | `translated-fixture-candidate`, `benchmark-candidate` | `NotComparable` | Offset metadata | `metadata-only-not-comparable` | Records query result IDs `{6,29,31,75}`, visitor semantics, and early-stop behavior. Performance treatment is deferred to Phase 4. |

## Deferred Evidence

| Evidence | Deferred to | Reason |
|----------|-------------|--------|
| Static spatial index throughput and benchmark profiles | Phase 4 | Requires benchmark baseline design and cost accounting. |
| Broad C++ offset/combine parameter import | Later absorption phases | Manual curated fixtures are enough for Phase 3; broad import risks red parity debugging. |
| FFI execution tests and generated header drift checks | API/FFI migration phase | C API is migration-sensitive evidence only here. |
| Clipper2 polygon-only oracle output | Phase 5 | Clipper2 eligibility and arc approximation policy are separate scope. |

## Harness Validation

| Gate | Status | Notes |
|------|--------|-------|
| `cargo test -p cavalier_contours --test test_historical_cavalier_contours` | pass | Executes `executable-green` offset/property fixtures and asserts `metadata-only-gap` / `metadata-only-not-comparable` records do not execute. |
| `cargo test -p cavalier_contours --test test_fixture_harness` | pass | Confirms Phase 2 seed fixtures still execute through the extended harness. |
| `cargo test --workspace` | pending final gate | Phase 03 final verification runs this after inventory/test synchronization. |

## Requirement Coverage

| Requirement | Coverage |
|-------------|----------|
| `FIX-03` | Inventories old C++ offset, combine/boolean, property, C API, and spatial-index evidence with provenance, classification, tolerance policy, and selected translated fixture IDs. |
