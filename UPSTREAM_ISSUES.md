# Upstream Issue Execution List

Snapshot date: 2026-05-16  
Source: `https://github.com/jbuckmccready/cavalier_contours/issues` (open issues list)

## Open Issues (12)

| Issue | Type | Title | Current Status (this branch) | Next Action |
| --- | --- | --- | --- | --- |
| #82 | bug | Empty offset of open polyline | Reproduced and fixed (`test_issue_82_open_polyline_negative_offset` passing) | Keep regression; include in final sweep |
| #79 | bug | assertion `left == right` failed | Reproduced with full upstream fixture and fixed (`test_issue_79_debug_assertion` passing) | Keep full regression in sweep |
| #43 | bug | one more data combination | Reproduced and fixed (`test_issue_43_boolean_and_empty` passing) | Keep regression; include in final sweep |
| #38 | bug | Cut not worked | Reproduced and fixed (`test_issue_38_cut_not_worked` passing) | Keep regression; include in final sweep |
| #72 | bug | Bool op seems not work with self intersects poly | Classified as unsupported input class (upstream maintainer confirmed); API docs updated | Optional: add helper utilities for splitting self-intersecting polylines into simple loops |
| #44 | question | how to handle offsets when they approximate a line | Classified and regression-covered (`issue_44_inward_offset_can_collapse_to_closed_overlapping_line`) | Keep behavior note and regression in final sweep |
| #39 | design decision | C FFI: Shouldn't counts be size_t? | Classified as fixed-width ABI choice; FFI docs updated | Keep `u32` C ABI unless explicitly taking a breaking ABI change |
| #35 | enhancement | offset vs. boolean op inconsistency? | Classified and regression-covered (`issue_35_boolean_union_merges_shared_edge_overlap`) | Keep known-limitation semantics and regression in final sweep |
| #76 | enhancement | Have an inverse function for seg_arc_radius_and_center | Implemented helper + regression coverage (`seg_arc_from_radius_center`, `test_issue_76_arc_from_radius_center`) | Keep API/docs/tests in final sweep |
| #73 | enhancement | A C# wrapper for cavalier_contours | Classified as ecosystem integration; docs updated with wrapper pointer | Keep link and wrapper policy in docs |
| #19 | design + enhancement | Support polyline with multiple offsets | Classified as deep algorithm/design gap (not a quick fix); limitation documented | Keep as future roadmap/design track |
| #80 | question | Wasm build | Classified and documented (UI wasm build + `default`/`initSync` expectation) | Keep docs note and build guidance |

## Execution Waves

1. **Wave 1 (critical correctness bugs):** #43, #38, #79  
2. **Wave 2 (behavior/compatibility):** #35, #39, #44 (triaged + documented + regression coverage)  
3. **Wave 3 (feature/docs):** #76 implemented; #80/#73/#19 documented and classified  
4. **Wave 4 (final verification):** `cargo test --workspace` passed; all issue regression tests in this branch are green.

## Immediate Commands

```powershell
cargo test -p cavalier_contours --test test_issue_82_open_polyline_negative_offset -- --nocapture
cargo test -p cavalier_contours --test test_issue_79_debug_assertion -- --nocapture
cargo test -p cavalier_contours --test test_issue_43_boolean_and_empty -- --nocapture
cargo test -p cavalier_contours --test test_issue_38_cut_not_worked -- --nocapture
cargo test -p cavalier_contours --test test_issue_35_44_overlap_semantics -- --nocapture
cargo test -p cavalier_contours --test test_issue_76_arc_from_radius_center -- --nocapture
```
