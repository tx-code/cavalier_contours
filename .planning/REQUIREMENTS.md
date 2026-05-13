# Requirements: Cavalier Contours Absorption Roadmap

**Defined:** 2026-05-12
**Core Value:** Make the Rust crate a robust, well-tested, arc-aware 2D geometry library whose behavior is defensible against historical CavalierContours behavior and polygon-only Clipper2 reference results.

## v1 Requirements

Requirements for the initial multi-milestone absorption roadmap. Each maps to
one roadmap phase.

### Audit

- [x] **AUD-01**: The project records a capability inventory across Rust `cavalier_contours`, old C++ CavalierContours, and Clipper2.
- [x] **AUD-02**: The project records license, provenance, and acceptable-use boundaries for mined tests, fixtures, benchmarks, and reference behavior.
- [x] **AUD-03**: The project defines a behavior taxonomy for exact parity, approximate parity, intentional divergence, and not-comparable cases.
- [x] **AUD-04**: The project compares public Rust APIs, C FFI surface, old C++ C API, and relevant Clipper2 operations for migration and compatibility planning.

### Fixtures

- [x] **FIX-01**: The project defines a durable fixture schema with source, geometry model, tolerance policy, comparison mode, and expected properties.
- [x] **FIX-02**: Tests can compare geometry by properties such as area, extents, path length, orientation, containment, repeat vertices, and result counts.
- [x] **FIX-03**: High-value old C++ tests and benchmark profiles are translated or represented as Rust regression fixtures.
- [x] **FIX-04**: Eligible Clipper2 polygon-only cases are represented as Rust fixtures with explicit comparability classification.

### Benchmarks

- [x] **BEN-01**: The project records a current Rust benchmark baseline for offsets, booleans, intersections, and spatial-index-heavy cases.
- [x] **BEN-02**: Historical old C++ benchmark profile families are mapped to Rust benchmark or measurement cases.
- [x] **BEN-03**: Benchmarks document whether arc approximation, conversion, and oracle execution costs are included or excluded.

### Oracle

- [x] **ORC-01**: A dev-only Clipper2 comparison path exists for eligible polygon-only boolean and offset cases.
- [x] **ORC-02**: Any arc-to-polygon comparison records approximation tolerance and does not redefine native arc behavior.
- [x] **ORC-03**: Oracle results are reported as evidence for gap ranking, not as automatic production behavior.

### Robustness

- [x] **ROB-01**: The project maintains a ranked robustness backlog for offsets, booleans, intersections, tolerances, degenerates, repeat vertices, tangencies, overlaps, and open/closed behavior.
- [x] **ROB-02**: Top-ranked robustness gaps have focused regression tests before or with fixes.
- [x] **ROB-03**: Top-ranked current Rust robustness issues are fixed without broad API churn.
- [x] **ROB-04**: Robustness phases pass the workspace verification gate required for the changed surface.

### Capability Absorption

- [x] **CAP-01**: Candidate capabilities from old C++ and Clipper2 are selected only after audit, fixtures, and gap ranking.
- [x] **CAP-02**: Each absorbed capability preserves the Rust crate's arc-aware model or explicitly documents why it is polygon-only.
- [x] **CAP-03**: Absorbed capabilities include tests, examples or docs, and FFI impact notes when externally visible.

### API and FFI

- [x] **API-01**: Public Rust API and FFI changes include explicit compatibility notes.
- [x] **API-02**: Any FFI surface change updates ABI tests and regenerates `cavalier_contours_ffi.h`.
- [x] **API-03**: The project produces migration notes for users coming from old C++ CavalierContours.

### Demo

- [x] **DEM-01**: The demo UI is updated only when a new or changed geometry capability needs visual validation.

## v2 Requirements

Deferred to a later roadmap. Tracked but not in current v1 scope.

### Geometry Expansion

- **GEO-01**: Add triangulation support after explicit rescoping and independent validation.
- **GEO-02**: Expand boolean operations beyond two closed non-self-intersecting polylines.
- **GEO-03**: Add additional offset join styles beyond rounded joins.
- **GEO-04**: Explore wider support for intersecting or open multi-polyline shape offsets.

### Tooling

- **TLG-01**: Automate large-scale import from old C++ and Clipper2 test corpora if manual translation becomes a bottleneck.
- **TLG-02**: Add generated/randomized differential test cases with shrinking or minimization.

### Product Surface

- **SUR-01**: Redesign the demo UI as a productized geometry workbench.
- **SUR-02**: Provide a production Clipper2 backend or runtime dependency.

### C++ Parity Deep Comparison

- **PAR-01**: The project maps old C++ logic modules/tests to Rust modules/tests for deep parity review without Clipper involvement.
- **PAR-02**: High-value C++ cases in boolean, offset, and intersection paths are executed or explicitly classified with evidence.
- **PAR-03**: Confirmed C++ vs Rust mismatches are classified as bug, intentional divergence, or not-comparable with recorded fix/defer decisions.
- **PAR-04**: The project maps old C++ `TEST_cavc_pline_function.cpp` function-level expectations to Rust function-level APIs and tests.
- **PAR-05**: Selected C++ function-level expectations (area/path/extents/winding/self-boolean invariants) execute in Rust parity tests with evidence.
- **PAR-06**: Newly surfaced function-level mismatches are classified with explicit fix/defer decisions.
- **PAR-07**: Closest-point expectations from old C++ `pline_function` cases are mapped into executable Rust parity checks with explicit index tie-break policy.
- **PAR-08**: A bounded subset of old C++ generated function-case matrices is executed in Rust parity tests or explicitly marked not-comparable.
- **PAR-09**: Closest-point and generated-matrix mismatches are classified with evidence and explicit fix/defer decisions.
- **PAR-10**: Full old C++ generated half-circle case matrices (open/closed, x/y-aligned, cw/ccw, multi-center) execute as Rust parity tests with reusable tolerance helpers.
- **PAR-11**: Closest-point expectations with explicit index results in generated half-circle cases are validated in strict mode and any tie-break mismatches are fixed or explicitly classified.
- **PAR-12**: Deep parity continuation includes a file/module alignment map that names next high-value C++ targets and Rust implementation surfaces.
- **PAR-13**: Full old C++ generated circle case matrices (all centers, alignments, reverse variants, and direction variants) execute as Rust parity tests with source-traceable expectations.
- **PAR-14**: Closest-point expectations from generated circle cases validate explicit index expectations in strict mode and keep non-explicit index cases as point/distance parity checks.
- **PAR-15**: After full circle matrix closure, a file/module alignment map names the next deep-parity targets for offset and collapsed-offset matrices.
- **PAR-16**: Full old C++ generated circle offset matrix expectations execute as Rust parity tests for outward and inward deltas across all generated variants.
- **PAR-17**: Generated circle collapsed-offset deltas from old C++ execute as Rust parity checks and remain empty where expected.
- **PAR-18**: Offset matrix parity validates both geometry properties and vertex-level output (with closed-curve start rotation tolerance) and publishes next-step alignment scope.
- **PAR-19**: Full old C++ generated half-circle offset matrix expectations execute as Rust parity tests for outward and inward deltas across all generated variants.
- **PAR-20**: Generated half-circle collapsed-offset deltas from old C++ execute as Rust parity checks and remain empty where expected.
- **PAR-21**: Half-circle offset parity validates both geometry properties and vertex-level output (open exact-order and closed-curve start rotation tolerance) and publishes next-step alignment scope.
- **PAR-22**: Old C++ `TEST_cavc_parallel_offset.cpp` simple and specific `parallel_offset` matrices execute as Rust parity tests with source-traceable expected property sets.
- **PAR-23**: Reversed-input parity (`invert_direction` + negated delta) executes across imported offset matrices with sign-adjusted area and matching geometric properties.
- **PAR-24**: Imported offset parity includes collapsed-result and input-immutability checks and publishes next deep-alignment scope.
- **PAR-25**: Old C++ `TEST_cavc_combine_plines.cpp` coincident case matrices execute as Rust parity tests across `Or`, `Not`, `And`, and `Xor` combine modes.
- **PAR-26**: Coincident combine outcomes are classified with explicit parity/divergence decisions and source-traceable evidence.
- **PAR-27**: Coincident combine phase closes with explicit next-target alignment map and full verification gate closure.
- **PAR-28**: Coincident intersect sliver behavior is covered by an explicit parity test path using `PlineBooleanOptions.collapsed_area_eps` that matches old C++ empty-output expectation.
- **PAR-29**: Default-path versus collapsed-filter-path behavior is explicitly classified and documented for the coincident intersect case.
- **PAR-30**: The project records the follow-up decision boundary for adopting or deferring a default collapsed-area threshold in boolean operations.
- **PAR-31**: Boolean stitching removes only degenerate line-only two-vertex closed loops while preserving valid two-vertex arc loops.
- **PAR-32**: `coincident_case1_intersect` default-path behavior matches old C++ empty-output parity through executable Rust tests.
- **PAR-33**: The project records the post-fix no-Clipper deep-alignment map and closes with full verification gates.
- **PAR-34**: The FFI surface (`cavc_pline_boolean`) includes an executable coincident intersect parity case sourced from old C++ combine inputs.
- **PAR-35**: FFI default-path `coincident_case1_intersect` behavior returns empty results for `And` operation parity with old C++ expectation.
- **PAR-36**: C-API parity bridge work records next C-API expansion scope and closes with full verification gates.
- **PAR-37**: The FFI surface executes full old C++ `circle_rectangle` combine matrix parity through `cavc_pline_boolean` with source-traceable expected properties.
- **PAR-38**: The FFI surface executes full old C++ `coincident_case2` combine matrix parity, including both exclude directions, with property-set matching.
- **PAR-39**: C-API combine matrix expansion records next C-API parity targets and closes with full verification gates.
- **PAR-40**: The FFI surface executes old C++ combine-with-self invariants for union/intersect/self-empty modes through `cavc_pline_boolean`.
- **PAR-41**: Reversed and mixed-orientation self-invariant empty-result cases are explicitly executed and verified at the C-API boundary.
- **PAR-42**: C-API self-invariants bridge work records next C-API parity scope and closes with full verification gates.
- **PAR-43**: The FFI surface executes old C++ `parallel_offset` simple and specific matrices through `cavc_pline_parallel_offset` with source-traceable expected properties.
- **PAR-44**: C-API `parallel_offset` reversed-input parity and no-modify input invariants are explicitly executed and verified.
- **PAR-45**: C-API parallel-offset bridge work records next C-API parity scope and closes with full verification gates.
- **PAR-46**: The FFI surface executes old C++ combine no-modify input invariants through `cavc_pline_boolean` operation matrix checks.
- **PAR-47**: C-API combine operation matrix no-modify checks validate both subject and clip vertex buffers remain unchanged.
- **PAR-48**: C-API combine no-modify bridge work records next C-API parity scope and closes with full verification gates.
- **PAR-49**: The FFI surface executes old C++ generated circle and half-circle function matrices for area/path/extents/winding through direct C-API calls.
- **PAR-50**: Closest-point expectations from old C++ function matrices are explicitly classified at C-API boundary as not-comparable until a closest-point FFI surface exists.
- **PAR-51**: C-API function-surface matrix bridge work records next parity target scope and closes with full verification gates.
- **PAR-52**: The FFI surface executes source-backed boolean matrix cases through `cavc_pline_boolean` options-path and matches default-path property outputs.
- **PAR-53**: The FFI surface executes source-backed offset matrix cases through `cavc_pline_parallel_offset` options-path and matches default-path property outputs.
- **PAR-54**: C-API options-path parity bridge work records next parity target scope and closes with full verification gates.
- **PAR-55**: The FFI surface executes source-backed coincident case1 and case2 boolean matrices through `cavc_pline_boolean` with no-modify subject/clip invariants.
- **PAR-56**: Coincident no-modify parity explicitly covers both exclusion directions (`A-B`, `B-A`) alongside union/intersect/xor checks.
- **PAR-57**: C-API coincident no-modify matrix expansion records next parity target scope and closes with full verification gates.
- **PAR-58**: The FFI surface executes coincident case1 intersect with `collapsed_area_eps` through `cavc_pline_boolean_o` and preserves empty-result parity.
- **PAR-59**: The FFI surface executes coincident case1/case2 options-path boolean matrices with subject/clip no-modify invariants.
- **PAR-60**: C-API optioned coincident edge parity work records next parity target scope and closes with full verification gates.
- **PAR-61**: The FFI surface executes source-backed coincident case1/case2 boolean matrices through default-path and options-path and validates output property-set parity.
- **PAR-62**: Options output parity explicitly covers exclusion direction variants (`A-B`, `B-A`) for coincident case matrices.
- **PAR-63**: C-API optioned coincident output parity work records next parity target scope and closes with full verification gates.
- **PAR-64**: The FFI surface exposes closest-point evaluation through `cavc_pline_eval_closest_point` with explicit null and empty-polyline behavior codes.
- **PAR-65**: Source-backed circle closest-point parity expectations execute through C-API (vertex anchors plus axis/45-degree probes) with index/point/distance checks.
- **PAR-66**: Closest-point C-API bridge updates ABI header surface and closes with full verification gates and next-scope alignment map.
- **PAR-67**: The FFI surface executes source-backed half-circle generated matrix closest-point probes through `cavc_pline_eval_closest_point` with strict index checks.
- **PAR-68**: Half-circle closest-point parity covers open/closed, x/y alignment, direction, and center variants with point/distance/index validation.
- **PAR-69**: Half-circle closest-point strict-index parity work records next parity target scope and closes with full verification gates.
- **PAR-70**: The FFI surface executes source-backed circle and closed half-circle function-surface matrix self-boolean invariants through `cavc_pline_boolean`.
- **PAR-71**: Function-surface self-boolean parity validates output vertex invariants (union/intersect keep self; exclude/xor empty) and input no-modify behavior.
- **PAR-72**: Function-surface combine-self matrix parity work records next parity target scope and closes with full verification gates.
- **PAR-73**: The FFI surface executes source-backed explicit closest-point index probes across an epsilon matrix through `cavc_pline_eval_closest_point`.
- **PAR-74**: Closest-point epsilon/tie-break parity validates circle shared-vertex and half-circle explicit-index stability for index/point/distance under epsilon variation.
- **PAR-75**: Closest-point epsilon/tie-break parity work records next parity target scope and closes with full verification gates.
- **PAR-76**: The FFI surface executes source-backed generated circle and half-circle full matrix outward/inward offset probes through `cavc_pline_parallel_offset`.
- **PAR-77**: Function-surface full-matrix offset parity validates vertex-level output semantics (closed rotational match, open exact order) and collapsed-delta empty results.
- **PAR-78**: Function-surface full-matrix parallel-offset parity work records next parity target scope and closes with full verification gates.
- **PAR-79**: The FFI surface validates source-backed nontrivial sample combine-with-self invariants at vertex-exact level through `cavc_pline_boolean`.
- **PAR-80**: Combine-self vertex-exact parity explicitly covers reversed self and reversed-forward cross-combination empty-result invariants for exclude/xor.
- **PAR-81**: Combine-self vertex-exact reversed parity work records next parity target scope and closes with full verification gates.
- **PAR-82**: The FFI surface preserves caller buffers on empty `cavc_pline_get_vertex_data` reads (source-backed pline-suite edge behavior).
- **PAR-83**: The FFI surface preserves existing vertex data across `cavc_pline_reserve` calls on populated polylines.
- **PAR-84**: Pline-suite buffer/reserve parity work records next parity target scope and closes with full verification gates.
- **PAR-85**: The FFI surface reproduces source-backed remove-range scenario behavior via ordered `cavc_pline_remove` calls on current API surface.
- **PAR-86**: Remove-sequence range-equivalence parity validates vertex-level intermediate transitions and final empty-state closure.
- **PAR-87**: Pline remove-sequence range-equivalence parity work records next parity target scope and closes with full verification gates.
- **PAR-88**: A cross-suite checklist maps old C++ C-API suite blocks (`pline`, `pline_function`, `parallel_offset`, `combine_plines`) to current FFI executable evidence.
- **PAR-89**: Cross-suite coverage audit explicitly classifies source-explicit uncovered or API-evolved equivalence zones with concrete notes.
- **PAR-90**: Cross-suite coverage audit closes with a post-audit alignment map and full verification/planning health gate closure.
- **PAR-91**: Reserve API-evolution equivalence remains guarded by regression tests covering shrink-noop/grow reserve calls with preserved existing vertex-prefix data.
- **PAR-92**: Remove-sequence range-equivalence regression includes final empty-state `cavc_pline_get_vertex_data` no-write buffer behavior in the same source-backed flow.
- **PAR-93**: Equivalence-zone hardening closes with full verification/planning health gates and explicit next-step alignment boundary.
- **PAR-94**: A drift baseline artifact tracks canonical old C++ suite files (pline, pline_function, parallel_offset, combine_plines) with source-root path, file hash, and test-block list.
- **PAR-95**: A deterministic executable hook command validates old-suite drift against baseline and fails on hash or test-block change.
- **PAR-96**: Drift-hook phase closes with full verification/planning health gates and an explicit post-hook alignment map.
- **PAR-97**: Options-path parallel-offset execution preserves input vertex data across source-backed simple/specific case matrices.
- **PAR-98**: Options-path boolean circle/rectangle matrix execution preserves both subject and clip input vertex data across union/exclude/intersect/xor operations.
- **PAR-99**: Options-path no-modify hardening closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-100**: Options-path boolean circle/rectangle matrix outputs are vertex-level equivalent to default-path outputs across union/exclude/intersect/xor (unordered polyline set, closed-rotation tolerant).
- **PAR-101**: Options-path parallel-offset simple/specific matrix outputs are vertex-level equivalent to default-path outputs (unordered set, closed/open-aware matching).
- **PAR-102**: Options-path vertex-output deepening closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-103**: A reusable drift-failure triage template exists with mandatory sections for drift evidence, old-suite block mapping, covered/equivalent/gap classification, and action decision.
- **PAR-104**: Drift-failure handling flow explicitly links drift-check command failure to deterministic triage and re-audit steps.
- **PAR-105**: Drift-failure triage-template phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-106**: Options-path coincident case1/case2 output matrices are vertex-level equivalent to default-path outputs across union/exclude/intersect/xor operations.
- **PAR-107**: Coincident options-path vertex-level parity explicitly validates both remaining and subtracted output sets.
- **PAR-108**: Coincident options-path vertex-output deepening closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-109**: Options-path boolean circle/rectangle matrix output remains equivalent to default-path output across bounded `pos_equal_eps` scale matrix.
- **PAR-110**: Options-path parallel-offset simple/specific matrix output remains equivalent to default-path output across bounded tolerance scale matrix (`pos_equal_eps`, `slice_join_eps`, `offset_dist_eps`).
- **PAR-111**: Options-path tolerance-matrix deepening closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-112**: Options-path `parallel_offset` output remains equivalent to default-path output across self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) for source-backed simple non-self-intersecting cases.
- **PAR-113**: Self-intersects mode matrix parity explicitly validates both property-level and vertex-level output equivalence.
- **PAR-114**: Self-intersects mode-matrix deepening closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-115**: Options-path self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) preserves input vertex data across source-backed simple and specific offset cases.
- **PAR-116**: Self-intersects mode no-modify matrix explicitly validates input stability per mode with source-backed case attribution.
- **PAR-117**: Self-intersects mode no-modify matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-118**: Options-path `parallel_offset` output remains equivalent to default-path output across self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`) for source-backed simple and specific cases.
- **PAR-119**: Self-intersects stress-matrix parity explicitly validates both property-level and vertex-level output equivalence with mode/scale attribution.
- **PAR-120**: Self-intersects stress-matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-121**: Reversed-input options-path `parallel_offset` output remains equivalent to reversed-input default-path output across self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`) for source-backed simple and specific cases.
- **PAR-122**: Reversed self-intersects stress-matrix parity explicitly validates both property-level and vertex-level output equivalence with mode/scale attribution.
- **PAR-123**: Reversed self-intersects stress-matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-124**: Reversed-input options-path `parallel_offset` execution preserves input vertex data across self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`) for source-backed simple and specific cases.
- **PAR-125**: Reversed self-intersects no-modify stress-matrix parity explicitly validates input stability with mode/scale attribution.
- **PAR-126**: Reversed self-intersects no-modify stress-matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-127**: FFI parity tests extract shared helper constructs for self-intersects include mode matrix and tolerance scale matrix setup in `test_pline.rs`.
- **PAR-128**: FFI parity helper extraction preserves behavior of existing options-path parity/no-modify assertions and keeps semantic outputs unchanged.
- **PAR-129**: FFI parity helper-extraction phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-130**: Reversed-input options-path stress matrix co-validates output parity and input no-modify invariants across self-intersects include modes and bounded tolerance scales on source-backed simple/specific offset cases.
- **PAR-131**: Merged reversed stress checks preserve explicit mode/scale-attributed failure diagnostics for both output and input-stability assertions.
- **PAR-132**: Reversed output/no-modify merge-matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-133**: Reversed-input source-backed specific-case matrix co-validates output parity and input no-modify invariants across self-intersects include modes and bounded tolerance scales.
- **PAR-134**: Specific-case parity checks include explicit old C++ provenance attribution in failure diagnostics for each imported specific scenario.
- **PAR-135**: Reversed specific-edge attribution matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-136**: Default-input options-path stress matrix co-validates output parity and input no-modify invariants across self-intersects include modes and bounded tolerance scales on source-backed simple/specific offset cases.
- **PAR-137**: Merged default-input stress checks preserve explicit mode/scale-attributed failure diagnostics for both output and input-stability assertions.
- **PAR-138**: Default output/no-modify merge-matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-139**: Default-input source-backed specific-case matrix co-validates output parity and input no-modify invariants across self-intersects include modes and bounded tolerance scales.
- **PAR-140**: Specific-case parity checks include explicit old C++ provenance attribution in failure diagnostics for each imported specific scenario.
- **PAR-141**: Default specific-edge attribution matrix phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-142**: Specific-edge attribution and matrix-execution helpers are extracted in `test_pline.rs` and reused by both reversed/default specific-edge options-path parity tests.
- **PAR-143**: Shared specific-edge helper extraction preserves existing parity/no-modify behavior and mode/scale/case-attributed diagnostics.
- **PAR-144**: Specific-edge runner-helper extraction phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-145**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ edge inputs beyond the original specific-case trio.
- **PAR-146**: Expanded specific-edge matrix coverage preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per edge case.
- **PAR-147**: Specific-edge matrix coverage expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-148**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ open-path case inputs beyond Phase 57 coverage.
- **PAR-149**: Open-path specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-150**: Specific-edge matrix open-path expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-151**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ diamond case inputs beyond Phase 58 coverage.
- **PAR-152**: Diamond specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-153**: Specific-edge matrix diamond expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-154**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ open-diamond case inputs beyond Phase 59 coverage.
- **PAR-155**: Open-diamond specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-156**: Specific-edge matrix open-diamond expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-157**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ open-diamond-outward case inputs beyond Phase 60 coverage.
- **PAR-158**: Open-diamond-outward specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-159**: Specific-edge matrix open-diamond-outward expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-160**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ closed-diamond-inward case inputs beyond Phase 61 coverage.
- **PAR-161**: Closed-diamond-inward specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-162**: Specific-edge matrix closed-diamond-inward expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-163**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ closed-rectangle-outward case inputs beyond Phase 62 coverage.
- **PAR-164**: Closed-rectangle-outward specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-165**: Specific-edge matrix closed-rectangle-outward expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-166**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ closed-rectangle-inward case inputs beyond Phase 63 coverage.
- **PAR-167**: Closed-rectangle-inward specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-168**: Specific-edge matrix closed-rectangle-inward expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-169**: Helper-driven specific-edge options-path matrix coverage includes additional source-backed old C++ open-rectangle-inward case inputs beyond Phase 64 coverage.
- **PAR-170**: Open-rectangle-inward specific-edge matrix expansion preserves reversed/default parity and no-modify diagnostics with explicit provenance labels per covered edge case.
- **PAR-171**: Specific-edge matrix open-rectangle-inward expansion phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-172**: Specific-edge matrix construction asserts that all source-backed old C++ simple cases targeted by this matrix path are consumed, and fails with omitted-case diagnostics when any remain.
- **PAR-173**: Source-coverage guard hardening preserves existing reversed/default parity and no-modify behavior while improving omission diagnostics.
- **PAR-174**: Specific-edge matrix source-coverage guard phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-175**: Coincident case1/case2 exclude metadata labels in C-API boolean matrix suites use old C++ canonical identifiers (`excludeAFromB`, `excludeBFromA`).
- **PAR-176**: Coincident exclude naming canonicalization preserves behavior of default/options/no-modify matrix assertions and expected outputs.
- **PAR-177**: Coincident exclude naming canonicalization phase closes with full verification/planning health gates and an explicit post-phase alignment map.
- **PAR-178**: C-API coincident matrix suites reuse one shared source-backed case helper for `name`, `operation`, `subject`, and `clip` metadata.
- **PAR-179**: Shared helper extraction preserves canonical naming, operation mapping, and behavioral assertions across default/options/no-modify/output matrix suites.
- **PAR-180**: C-API coincident matrix helper extraction phase closes with full verification/planning health gates and an explicit post-phase alignment map.

## Out of Scope

Explicitly excluded to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Triangulation | User explicitly deferred it, and Clipper2 README warns its triangulation code is buggy. |
| Clipper2 as production backend | Clipper2 is polygon-focused and should be an oracle/reference, not a replacement for the arc-aware Rust kernel. |
| Port-first algorithm work | Absorption must begin with audit, fixtures, benchmarks, and evidence. |
| Broad UI redesign | UI changes are tied only to new functionality that needs demo or validation support. |
| Unsafe code in the core crate | Current crate-level policy forbids unsafe in the core library. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| AUD-01 | Phase 1 | Complete |
| AUD-02 | Phase 1 | Complete |
| AUD-03 | Phase 1 | Complete |
| AUD-04 | Phase 1 | Complete |
| FIX-01 | Phase 2 | Complete |
| FIX-02 | Phase 2 | Complete |
| FIX-03 | Phase 3 | Complete |
| FIX-04 | Phase 5 | Complete |
| BEN-01 | Phase 4 | Complete |
| BEN-02 | Phase 4 | Complete |
| BEN-03 | Phase 4 | Complete |
| ORC-01 | Phase 5 | Complete |
| ORC-02 | Phase 5 | Complete |
| ORC-03 | Phase 5 | Complete |
| ROB-01 | Phase 6 | Complete |
| ROB-02 | Phase 6 | Complete |
| ROB-03 | Phase 6 | Complete |
| ROB-04 | Phase 6 | Complete |
| CAP-01 | Phase 7 | Complete |
| CAP-02 | Phase 7 | Complete |
| CAP-03 | Phase 7 | Complete |
| API-01 | Phase 8 | Complete |
| API-02 | Phase 8 | Complete |
| API-03 | Phase 8 | Complete |
| DEM-01 | Phase 7 | Complete |
| PAR-01 | Phase 9 | Complete |
| PAR-02 | Phase 9 | Complete |
| PAR-03 | Phase 9 | Complete |
| PAR-04 | Phase 10 | Complete |
| PAR-05 | Phase 10 | Complete |
| PAR-06 | Phase 10 | Complete |
| PAR-07 | Phase 11 | Complete |
| PAR-08 | Phase 11 | Complete |
| PAR-09 | Phase 11 | Complete |
| PAR-10 | Phase 12 | Complete |
| PAR-11 | Phase 12 | Complete |
| PAR-12 | Phase 12 | Complete |
| PAR-13 | Phase 13 | Complete |
| PAR-14 | Phase 13 | Complete |
| PAR-15 | Phase 13 | Complete |
| PAR-16 | Phase 14 | Complete |
| PAR-17 | Phase 14 | Complete |
| PAR-18 | Phase 14 | Complete |
| PAR-19 | Phase 15 | Complete |
| PAR-20 | Phase 15 | Complete |
| PAR-21 | Phase 15 | Complete |
| PAR-22 | Phase 16 | Complete |
| PAR-23 | Phase 16 | Complete |
| PAR-24 | Phase 16 | Complete |
| PAR-25 | Phase 17 | Complete |
| PAR-26 | Phase 17 | Complete |
| PAR-27 | Phase 17 | Complete |
| PAR-28 | Phase 18 | Complete |
| PAR-29 | Phase 18 | Complete |
| PAR-30 | Phase 18 | Complete |
| PAR-31 | Phase 19 | Complete |
| PAR-32 | Phase 19 | Complete |
| PAR-33 | Phase 19 | Complete |
| PAR-34 | Phase 20 | Complete |
| PAR-35 | Phase 20 | Complete |
| PAR-36 | Phase 20 | Complete |
| PAR-37 | Phase 21 | Complete |
| PAR-38 | Phase 21 | Complete |
| PAR-39 | Phase 21 | Complete |
| PAR-40 | Phase 22 | Complete |
| PAR-41 | Phase 22 | Complete |
| PAR-42 | Phase 22 | Complete |
| PAR-43 | Phase 23 | Complete |
| PAR-44 | Phase 23 | Complete |
| PAR-45 | Phase 23 | Complete |
| PAR-46 | Phase 24 | Complete |
| PAR-47 | Phase 24 | Complete |
| PAR-48 | Phase 24 | Complete |
| PAR-49 | Phase 25 | Complete |
| PAR-50 | Phase 25 | Complete |
| PAR-51 | Phase 25 | Complete |
| PAR-52 | Phase 26 | Complete |
| PAR-53 | Phase 26 | Complete |
| PAR-54 | Phase 26 | Complete |
| PAR-55 | Phase 27 | Complete |
| PAR-56 | Phase 27 | Complete |
| PAR-57 | Phase 27 | Complete |
| PAR-58 | Phase 28 | Complete |
| PAR-59 | Phase 28 | Complete |
| PAR-60 | Phase 28 | Complete |
| PAR-61 | Phase 29 | Complete |
| PAR-62 | Phase 29 | Complete |
| PAR-63 | Phase 29 | Complete |
| PAR-64 | Phase 30 | Complete |
| PAR-65 | Phase 30 | Complete |
| PAR-66 | Phase 30 | Complete |
| PAR-67 | Phase 31 | Complete |
| PAR-68 | Phase 31 | Complete |
| PAR-69 | Phase 31 | Complete |
| PAR-70 | Phase 32 | Complete |
| PAR-71 | Phase 32 | Complete |
| PAR-72 | Phase 32 | Complete |
| PAR-73 | Phase 33 | Complete |
| PAR-74 | Phase 33 | Complete |
| PAR-75 | Phase 33 | Complete |
| PAR-76 | Phase 34 | Complete |
| PAR-77 | Phase 34 | Complete |
| PAR-78 | Phase 34 | Complete |
| PAR-79 | Phase 35 | Complete |
| PAR-80 | Phase 35 | Complete |
| PAR-81 | Phase 35 | Complete |
| PAR-82 | Phase 36 | Complete |
| PAR-83 | Phase 36 | Complete |
| PAR-84 | Phase 36 | Complete |
| PAR-85 | Phase 37 | Complete |
| PAR-86 | Phase 37 | Complete |
| PAR-87 | Phase 37 | Complete |
| PAR-88 | Phase 38 | Complete |
| PAR-89 | Phase 38 | Complete |
| PAR-90 | Phase 38 | Complete |
| PAR-91 | Phase 39 | Complete |
| PAR-92 | Phase 39 | Complete |
| PAR-93 | Phase 39 | Complete |
| PAR-94 | Phase 40 | Complete |
| PAR-95 | Phase 40 | Complete |
| PAR-96 | Phase 40 | Complete |
| PAR-97 | Phase 41 | Complete |
| PAR-98 | Phase 41 | Complete |
| PAR-99 | Phase 41 | Complete |
| PAR-100 | Phase 42 | Complete |
| PAR-101 | Phase 42 | Complete |
| PAR-102 | Phase 42 | Complete |
| PAR-103 | Phase 43 | Complete |
| PAR-104 | Phase 43 | Complete |
| PAR-105 | Phase 43 | Complete |
| PAR-106 | Phase 44 | Complete |
| PAR-107 | Phase 44 | Complete |
| PAR-108 | Phase 44 | Complete |
| PAR-109 | Phase 45 | Complete |
| PAR-110 | Phase 45 | Complete |
| PAR-111 | Phase 45 | Complete |
| PAR-112 | Phase 46 | Complete |
| PAR-113 | Phase 46 | Complete |
| PAR-114 | Phase 46 | Complete |
| PAR-115 | Phase 47 | Complete |
| PAR-116 | Phase 47 | Complete |
| PAR-117 | Phase 47 | Complete |
| PAR-118 | Phase 48 | Complete |
| PAR-119 | Phase 48 | Complete |
| PAR-120 | Phase 48 | Complete |
| PAR-121 | Phase 49 | Complete |
| PAR-122 | Phase 49 | Complete |
| PAR-123 | Phase 49 | Complete |
| PAR-124 | Phase 50 | Complete |
| PAR-125 | Phase 50 | Complete |
| PAR-126 | Phase 50 | Complete |
| PAR-127 | Phase 51 | Complete |
| PAR-128 | Phase 51 | Complete |
| PAR-129 | Phase 51 | Complete |
| PAR-130 | Phase 52 | Complete |
| PAR-131 | Phase 52 | Complete |
| PAR-132 | Phase 52 | Complete |
| PAR-133 | Phase 53 | Complete |
| PAR-134 | Phase 53 | Complete |
| PAR-135 | Phase 53 | Complete |
| PAR-136 | Phase 54 | Complete |
| PAR-137 | Phase 54 | Complete |
| PAR-138 | Phase 54 | Complete |
| PAR-139 | Phase 55 | Complete |
| PAR-140 | Phase 55 | Complete |
| PAR-141 | Phase 55 | Complete |
| PAR-142 | Phase 56 | Complete |
| PAR-143 | Phase 56 | Complete |
| PAR-144 | Phase 56 | Complete |
| PAR-145 | Phase 57 | Complete |
| PAR-146 | Phase 57 | Complete |
| PAR-147 | Phase 57 | Complete |
| PAR-148 | Phase 58 | Complete |
| PAR-149 | Phase 58 | Complete |
| PAR-150 | Phase 58 | Complete |
| PAR-151 | Phase 59 | Complete |
| PAR-152 | Phase 59 | Complete |
| PAR-153 | Phase 59 | Complete |
| PAR-154 | Phase 60 | Complete |
| PAR-155 | Phase 60 | Complete |
| PAR-156 | Phase 60 | Complete |
| PAR-157 | Phase 61 | Complete |
| PAR-158 | Phase 61 | Complete |
| PAR-159 | Phase 61 | Complete |
| PAR-160 | Phase 62 | Complete |
| PAR-161 | Phase 62 | Complete |
| PAR-162 | Phase 62 | Complete |
| PAR-163 | Phase 63 | Complete |
| PAR-164 | Phase 63 | Complete |
| PAR-165 | Phase 63 | Complete |
| PAR-166 | Phase 64 | Complete |
| PAR-167 | Phase 64 | Complete |
| PAR-168 | Phase 64 | Complete |
| PAR-169 | Phase 65 | Complete |
| PAR-170 | Phase 65 | Complete |
| PAR-171 | Phase 65 | Complete |
| PAR-172 | Phase 66 | Complete |
| PAR-173 | Phase 66 | Complete |
| PAR-174 | Phase 66 | Complete |
| PAR-175 | Phase 67 | Complete |
| PAR-176 | Phase 67 | Complete |
| PAR-177 | Phase 67 | Complete |
| PAR-178 | Phase 68 | Complete |
| PAR-179 | Phase 68 | Complete |
| PAR-180 | Phase 68 | Complete |

**Coverage:**
- v1 requirements: 25 total
- v1 mapped to phases: 25
- v1 unmapped: 0
- additional tracked post-v1 requirements: 180 (`PAR-01..PAR-180`), mapped to Phases 9-68

---
*Requirements defined: 2026-05-12*
*Last updated: 2026-05-14 after Phase 68 completion*
