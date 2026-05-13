# Phase 38: Cross-Suite Coverage Checklist

This checklist maps old C++ C-API suite blocks to concrete Rust FFI evidence.

## Coverage Matrix

| Old C++ suite block | Current FFI evidence | Status | Notes |
|---|---|---|---|
| `TEST_cavc_pline.cpp::cavc_pline_new` | `pline_data_manipulation`, `pline_get_vertex_data_empty_does_not_modify_buffer_cpp_parity` | covered | Includes creation, vertex readback, empty-buffer no-write behavior. |
| `TEST_cavc_pline.cpp::cavc_pline_set_capacity` | `pline_reserve_does_not_modify_existing_vertex_data_cpp_parity` | equivalent | API evolved to `reserve`; no direct capacity getter, behavior preserved via non-modify assertions. |
| `TEST_cavc_pline.cpp::cavc_pline_set_vertex_data` | `pline_data_manipulation` | covered | Set/get vertex data and value assertions present. |
| `TEST_cavc_pline.cpp::cavc_pline_add_vertex` | `pline_data_manipulation` | covered | Add and readback behavior asserted. |
| `TEST_cavc_pline.cpp::cavc_pline_remove_range` | `pline_remove_sequence_equivalent_to_cpp_remove_range_parity` | equivalent | Current API uses single remove; ordered sequence reproduces same scenario semantics. |
| `TEST_cavc_pline.cpp::cavc_pline_clear` | `pline_data_manipulation` | covered | Clear and zero-count closure asserted. |
| `TEST_cavc_pline_function.cpp::metrics/winding/extents` | `pline_function_surface_circle_metrics_winding_cpp_matrix_parity`, `pline_function_surface_half_circle_metrics_winding_cpp_matrix_parity` | covered | Full generated matrices covered. |
| `TEST_cavc_pline_function.cpp::closest_point` | `pline_function_surface_circle_closest_point_cpp_matrix_parity`, `pline_function_surface_half_circle_closest_point_strict_index_cpp_matrix_parity`, epsilon/tie-break tests | covered | Explicit index probes + epsilon matrix stability covered. |
| `TEST_cavc_pline_function.cpp::parallel_offset` | function-surface full-matrix offset/collapsed tests | covered | Circle + half-circle outward/inward/collapsed matrix behavior covered. |
| `TEST_cavc_pline_function.cpp::combine_with_self_invariants` | `pline_function_surface_closed_matrix_combine_with_self_cpp_parity`, `pline_boolean_combine_with_self_invariants_vertex_exact_cpp_parity` | covered | Property-level + vertex-exact strengthening. |
| `TEST_cavc_parallel_offset.cpp::parallel_offset_test` | `pline_parallel_offset_cpp_simple_matrix_parity`, `pline_parallel_offset_cpp_specific_matrix_parity` | covered | Property-set outputs matched by case matrix. |
| `TEST_cavc_parallel_offset.cpp::reversed_parallel_offset_test` | `pline_parallel_offset_cpp_reversed_matrix_parity` | covered | Reversed input and negated delta parity covered. |
| `TEST_cavc_parallel_offset.cpp::does_not_modify_input_test` | `pline_parallel_offset_does_not_modify_input_cpp_parity` | covered | Input no-modify invariant explicitly asserted. |
| `TEST_cavc_combine_plines.cpp::combine_plines_test` | circle/rectangle/coincident matrix parity tests + options-path output parity | covered | Union/exclude/intersect/xor matrices covered. |
| `TEST_cavc_combine_plines.cpp::combine_with_self_invariants` | `pline_boolean_combine_with_self_invariants_cpp_parity`, vertex-exact reversed test | covered | Forward/reversed/self-cross invariants covered. |
| `TEST_cavc_combine_plines.cpp::does_not_modify_input` | `pline_boolean_does_not_modify_input_cpp_parity`, coincident no-modify tests | covered | Subject/clip no-modify invariants covered across case classes. |

## Residual Gaps

No hard uncovered source-explicit C++ suite blocks were found in this audit.

## Follow-up Boundaries

- Keep future additions source-explicit only.
- If old C++ suites add/modify blocks, re-run this checklist before claiming closure.
