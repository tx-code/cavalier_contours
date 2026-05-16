use cavalier_contours::polyline::{BooleanOp, PlineSource, PlineSourceMut, Polyline};

fn rectangle(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Polyline<f64> {
    let mut pl = Polyline::new_closed();
    pl.add(min_x, min_y, 0.0);
    pl.add(max_x, min_y, 0.0);
    pl.add(max_x, max_y, 0.0);
    pl.add(min_x, max_y, 0.0);
    pl
}

#[test]
fn issue_35_boolean_union_merges_shared_edge_overlap() {
    // Upstream issue #35 highlights behavior differences:
    // - parallel offset keeps overlapping sections,
    // - boolean union merges/removes them.
    let left = rectangle(0.0, 0.0, 10.0, 10.0);
    let right = rectangle(10.0, 0.0, 20.0, 10.0);

    let result = left.boolean(&right, BooleanOp::Or);
    assert_eq!(result.pos_plines.len(), 1, "expected merged union result");
    assert!(result.neg_plines.is_empty());

    let merged = &result.pos_plines[0].pline;
    assert!(
        merged.remove_repeat_pos(1e-5).is_none(),
        "boolean union should not preserve the shared-overlap edge as a repeated segment"
    );
    assert!(
        (merged.area().abs() - 200.0).abs() < 1e-4,
        "unexpected merged area: {}",
        merged.area()
    );
}

#[test]
fn issue_44_inward_offset_can_collapse_to_closed_overlapping_line() {
    // Upstream issue #44: inward offsets near collapse may return a closed path that
    // geometrically approximates a line (not an open polyline).
    let input = rectangle(0.0, 0.0, 20.0, 10.0);
    let offset_result = input.parallel_offset(5.0);

    assert_eq!(offset_result.len(), 1);
    let collapsed = &offset_result[0];
    assert!(collapsed.is_closed());
    assert_eq!(collapsed.vertex_count(), 2);
    assert!(collapsed.area().abs() < 1e-4);
}
