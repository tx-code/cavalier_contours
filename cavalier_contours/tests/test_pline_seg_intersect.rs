use cavalier_contours::{
    core::math::{Vector2, bulge_from_angle},
    polyline::{PlineSegIntr::*, PlineVertex, pline_seg_intr},
};
use std::f64::consts::FRAC_PI_2;

macro_rules! assert_case_eq {
    ($left:expr, $right:expr) => {
        match ($left, $right) {
            (NoIntersect, NoIntersect) => {}
            (TangentIntersect { point: a1 }, TangentIntersect { point: a2 })
            | (OneIntersect { point: a1 }, OneIntersect { point: a2 })
                if a1.fuzzy_eq(a2) => {}
            (
                TwoIntersects {
                    point1: a1,
                    point2: b1,
                },
                TwoIntersects {
                    point1: a2,
                    point2: b2,
                },
            )
            | (
                OverlappingLines {
                    point1: a1,
                    point2: b1,
                },
                OverlappingLines {
                    point1: a2,
                    point2: b2,
                },
            )
            | (
                OverlappingArcs {
                    point1: a1,
                    point2: b1,
                },
                OverlappingArcs {
                    point1: a2,
                    point2: b2,
                },
            ) if a1.fuzzy_eq(a2) && b1.fuzzy_eq(b2) => {}
            _ => panic!(
                "intersect cases do not match: left: {:?}, right: {:?}",
                $left, $right
            ),
        };
    };
}

#[test]
fn arc_line_no_intersect() {
    let v1 = PlineVertex::new(0.0, 0.0, 1.0);
    let v2 = PlineVertex::new(2.0, 0.0, 0.0);
    let u1 = PlineVertex::new(0.0, 1.0, 0.0);
    let u2 = PlineVertex::new(2.0, 3.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(result, NoIntersect::<f64>);
}

#[test]
fn line_arc_no_intersect() {
    let v1 = PlineVertex::new(0.0, 1.0, 0.0);
    let v2 = PlineVertex::new(2.0, 3.0, 0.0);
    let u1 = PlineVertex::new(0.0, 0.0, 1.0);
    let u2 = PlineVertex::new(2.0, 0.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(result, NoIntersect::<f64>);
}

#[test]
fn overlapping_lines() {
    let v1 = PlineVertex::new(3.0, 3.0, 0.0);
    let v2 = PlineVertex::new(1.0, 1.0, 0.0);
    let u1 = PlineVertex::new(1.0, 1.0, 0.0);
    let u2 = PlineVertex::new(2.0, 2.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingLines {
            point1: Vector2::new(1.0, 1.0),
            point2: Vector2::new(2.0, 2.0)
        }
    );
}

#[test]
fn overlapping_lines_reverse_dir() {
    let v1 = PlineVertex::new(1.0, 1.0, 0.0);
    let v2 = PlineVertex::new(3.0, 3.0, 0.0);
    let u1 = PlineVertex::new(2.0, 2.0, 0.0);
    let u2 = PlineVertex::new(1.0, 1.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingLines {
            point1: Vector2::new(2.0, 2.0),
            point2: Vector2::new(1.0, 1.0)
        }
    );
}

#[test]
fn overlapping_same_arcs() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 3.0, 0.0);
    let u1 = PlineVertex::new(1.0, 1.0, 1.0);
    let u2 = PlineVertex::new(3.0, 3.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(1.0, 1.0),
            point2: Vector2::new(3.0, 3.0)
        }
    );
}

#[test]
fn overlapping_same_arcs_reverse_dir() {
    let v1 = PlineVertex::new(3.0, 3.0, -1.0);
    let v2 = PlineVertex::new(1.0, 1.0, 0.0);
    let u1 = PlineVertex::new(1.0, 1.0, 1.0);
    let u2 = PlineVertex::new(3.0, 3.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(1.0, 1.0),
            point2: Vector2::new(3.0, 3.0)
        }
    );
}

#[test]
fn arc_arc_end_points_touch() {
    let v1 = PlineVertex::new(3.0, 3.0, 1.0);
    let v2 = PlineVertex::new(1.0, 1.0, 0.0);
    let u1 = PlineVertex::new(1.0, 1.0, 1.0);
    let u2 = PlineVertex::new(3.0, 3.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        TwoIntersects {
            point1: Vector2::new(1.0, 1.0),
            point2: Vector2::new(3.0, 3.0)
        }
    );
}

#[test]
fn arc_arc_end_points_touch_reverse_dir() {
    let v1 = PlineVertex::new(1.0, 1.0, -1.0);
    let v2 = PlineVertex::new(3.0, 3.0, 0.0);
    let u1 = PlineVertex::new(1.0, 1.0, 1.0);
    let u2 = PlineVertex::new(3.0, 3.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        TwoIntersects {
            point1: Vector2::new(1.0, 1.0),
            point2: Vector2::new(3.0, 3.0)
        }
    );

    // reverse parameter order should yield the same result
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        TwoIntersects {
            point1: Vector2::new(1.0, 1.0),
            point2: Vector2::new(3.0, 3.0)
        }
    );

    // changing direction of arc2 should yield the same result BUT point1/point2 ordered according to
    // second segment direction
    let u1 = PlineVertex::new(3.0, 3.0, -1.0);
    let u2 = PlineVertex::new(1.0, 1.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        TwoIntersects {
            point1: Vector2::new(3.0, 3.0),
            point2: Vector2::new(1.0, 1.0)
        }
    );
}

#[test]
fn arc_arc_coincident_touch_only_at_arc1_start() {
    // Source-aligned with old C++ `intrPlineSegs` coincident-arc branch where
    // arc2 end angle equals arc1 start angle, yielding exactly one endpoint intersect.
    let quarter = bulge_from_angle(FRAC_PI_2);
    let v1 = PlineVertex::new(1.0, 0.0, quarter);
    let v2 = PlineVertex::new(0.0, 1.0, 0.0);
    let u1 = PlineVertex::new(0.0, -1.0, quarter);
    let u2 = PlineVertex::new(1.0, 0.0, 0.0);

    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OneIntersect {
            point: Vector2::new(1.0, 0.0)
        }
    );
}

#[test]
fn arc_arc_coincident_touch_only_at_arc2_start() {
    // Source-aligned with old C++ `intrPlineSegs` coincident-arc branch where
    // arc2 start angle equals arc1 end angle, yielding exactly one endpoint intersect.
    let quarter = bulge_from_angle(FRAC_PI_2);
    let v1 = PlineVertex::new(1.0, 0.0, quarter);
    let v2 = PlineVertex::new(0.0, 1.0, 0.0);
    let u1 = PlineVertex::new(0.0, 1.0, quarter);
    let u2 = PlineVertex::new(-1.0, 0.0, 0.0);

    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OneIntersect {
            point: Vector2::new(0.0, 1.0)
        }
    );
}

#[test]
fn arc2_within_arc1_overlapping() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let bulge = bulge_from_angle(FRAC_PI_2);
    let u1 = PlineVertex::new(2.0, 0.0, bulge);
    let u2 = PlineVertex::new(3.0, 1.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(2.0, 0.0),
            point2: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn arc1_within_arc2_overlapping() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let bulge = bulge_from_angle(FRAC_PI_2);
    let u1 = PlineVertex::new(2.0, 0.0, bulge);
    let u2 = PlineVertex::new(3.0, 1.0, 0.0);
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(2.0, 0.0),
            point2: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn arc2_within_arc1_overlapping_reverse_dir() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let bulge = bulge_from_angle(FRAC_PI_2);
    let u1 = PlineVertex::new(3.0, 1.0, -bulge);
    let u2 = PlineVertex::new(2.0, 0.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(3.0, 1.0),
            point2: Vector2::new(2.0, 0.0)
        }
    );
}

#[test]
fn arc1_within_arc2_overlapping_reverse_dir() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let bulge = bulge_from_angle(FRAC_PI_2);
    let u1 = PlineVertex::new(3.0, 1.0, -bulge);
    let u2 = PlineVertex::new(2.0, 0.0, 0.0);
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(2.0, 0.0),
            point2: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 0.0, 1.0);
    let u2 = PlineVertex::new(2.0, 2.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(2.0, 0.0),
            point2: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap_flipped() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 0.0, 1.0);
    let u2 = PlineVertex::new(2.0, 2.0, 0.0);
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(2.0, 0.0),
            point2: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap_arc2_reverse_dir() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 2.0, -1.0);
    let u2 = PlineVertex::new(2.0, 0.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(3.0, 1.0),
            point2: Vector2::new(2.0, 0.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap_arc2_reverse_dir_flipped() {
    let v1 = PlineVertex::new(1.0, 1.0, 1.0);
    let v2 = PlineVertex::new(3.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 2.0, -1.0);
    let u2 = PlineVertex::new(2.0, 0.0, 0.0);
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(2.0, 0.0),
            point2: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap_arc1_reverse_dir() {
    let v1 = PlineVertex::new(3.0, 1.0, -1.0);
    let v2 = PlineVertex::new(1.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 0.0, 1.0);
    let u2 = PlineVertex::new(2.0, 2.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(2.0, 0.0),
            point2: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap_arc1_reverse_dir_flipped() {
    let v1 = PlineVertex::new(3.0, 1.0, -1.0);
    let v2 = PlineVertex::new(1.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 0.0, 1.0);
    let u2 = PlineVertex::new(2.0, 2.0, 0.0);
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(3.0, 1.0),
            point2: Vector2::new(2.0, 0.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap_both_reverse_dir() {
    // Source-aligned with old C++ `intrPlineSegs` coincident-arc overlap path where
    // both arcs are reversed relative to the base partial-overlap geometry.
    let v1 = PlineVertex::new(3.0, 1.0, -1.0);
    let v2 = PlineVertex::new(1.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 2.0, -1.0);
    let u2 = PlineVertex::new(2.0, 0.0, 0.0);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(3.0, 1.0),
            point2: Vector2::new(2.0, 0.0)
        }
    );
}

#[test]
fn arc_arc_partial_overlap_both_reverse_dir_flipped() {
    // With both arcs reversed, swapping parameter order does not flip overlap endpoint
    // ordering for this bounded non-circle overlap geometry.
    let v1 = PlineVertex::new(3.0, 1.0, -1.0);
    let v2 = PlineVertex::new(1.0, 1.0, 0.0);

    let u1 = PlineVertex::new(2.0, 2.0, -1.0);
    let u2 = PlineVertex::new(2.0, 0.0, 0.0);
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingArcs {
            point1: Vector2::new(3.0, 1.0),
            point2: Vector2::new(2.0, 0.0)
        }
    );
}

#[test]
fn arc_arc_opposite_direction_touch_at_ends_bug() {
    // This test case reproduces the bug where arcs have the same radius and center but opposite
    // directions and only touch at the end points.
    // The bug was that when same_direction_arcs = false, the code would return u1.pos()
    // as the intersection point, but after direction adjustment, u1.pos() is actually
    // the END of arc2, not the start. The actual intersection should be at u2.pos().
    //
    // Original issue that found it: https://github.com/jbuckmccready/cavalier_contours/issues/42

    // Arc1
    let v1 = PlineVertex::new(-189.0, -196.91384910249, 0.553407781718062);
    let v2 = PlineVertex::new(-170.999999999999, -225.631646989572, -0.553407781718061);

    // Arc2
    let u1 = PlineVertex::new(-153.0, -196.91384910249, -0.553407781718061);
    let u2 = PlineVertex::new(-171.0, -225.631646989571, -0.553407781718061);

    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);

    // The arcs should intersect at u2.pos() (where arc1 and arc2 ends),
    // NOT at u1.pos() (which is ~34 units away from the actual intersection)
    assert_case_eq!(
        result,
        OneIntersect {
            point: Vector2::new(-171.0, -225.631646989571) // u2.pos()
        }
    );

    // reverse parameter order should yield the same result
    let result = pline_seg_intr(u1, u2, v1, v2, 1e-5);
    assert_case_eq!(
        result,
        OneIntersect {
            point: Vector2::new(-171.0, -225.631646989571) // u2.pos()
        }
    );

    // changing direction of arc2 should yield the same result
    let u1 = PlineVertex::new(-171.0, -225.631646989571, 0.553407781718062);
    let u2 = PlineVertex::new(-153.0, -196.91384910249, -0.553407781718061);
    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OneIntersect {
            point: Vector2::new(-171.0, -225.631646989571)
        }
    );
}

#[test]
fn cpp_pline_seg_line_line_overlap_order_parity() {
    // Source-aligned with old C++ `intrPlineSegs` line-line overlap path:
    // overlap points are returned in second segment direction.
    let v1 = PlineVertex::new(0.0, 0.0, 0.0);
    let v2 = PlineVertex::new(4.0, 0.0, 0.0);
    let u1 = PlineVertex::new(3.0, 0.0, 0.0);
    let u2 = PlineVertex::new(1.0, 0.0, 0.0);

    let result = pline_seg_intr(v1, v2, u1, u2, 1e-5);
    assert_case_eq!(
        result,
        OverlappingLines {
            point1: Vector2::new(3.0, 0.0),
            point2: Vector2::new(1.0, 0.0)
        }
    );
}

#[test]
fn cpp_pline_seg_line_arc_endpoint_sticky_parity() {
    // Source-aligned with old C++ `intrPlineSegs` line-arc path where line-circle has
    // two solutions but only one lies on the arc sweep and the line endpoint is sticky.
    let line_start = PlineVertex::new(3.0, 1.0, 0.0);
    let line_end = PlineVertex::new(1.0, 1.0, 0.0);
    let arc_bulge = bulge_from_angle(FRAC_PI_2);
    let arc_start = PlineVertex::new(2.0, 0.0, arc_bulge);
    let arc_end = PlineVertex::new(3.0, 1.0, 0.0);

    let result = pline_seg_intr(line_start, line_end, arc_start, arc_end, 1e-5);
    assert_case_eq!(
        result,
        OneIntersect {
            point: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn cpp_pline_seg_arc_line_endpoint_sticky_parity() {
    // Same geometry as line-arc sticky test but with arc as first segment and line as second
    // segment to verify the symmetric `u_is_line` path.
    let arc_bulge = bulge_from_angle(FRAC_PI_2);
    let arc_start = PlineVertex::new(2.0, 0.0, arc_bulge);
    let arc_end = PlineVertex::new(3.0, 1.0, 0.0);
    let line_start = PlineVertex::new(3.0, 1.0, 0.0);
    let line_end = PlineVertex::new(1.0, 1.0, 0.0);

    let result = pline_seg_intr(arc_start, arc_end, line_start, line_end, 1e-5);
    assert_case_eq!(
        result,
        OneIntersect {
            point: Vector2::new(3.0, 1.0)
        }
    );
}

#[test]
fn cpp_pline_seg_line_arc_two_intersects_second_arc_direction_order() {
    // Source-aligned with old C++ `intrPlineSegs` two-intersect line-arc path:
    // output order follows second segment (arc) direction.
    let half_sqrt_3 = 3.0_f64.sqrt() / 2.0;
    let line_start = PlineVertex::new(0.0, 0.5, 0.0);
    let line_end = PlineVertex::new(4.0, 0.5, 0.0);
    let arc_start = PlineVertex::new(3.0, 1.0, -1.0);
    let arc_end = PlineVertex::new(1.0, 1.0, 0.0);

    let result = pline_seg_intr(line_start, line_end, arc_start, arc_end, 1e-5);
    assert_case_eq!(
        result,
        TwoIntersects {
            point1: Vector2::new(2.0 + half_sqrt_3, 0.5),
            point2: Vector2::new(2.0 - half_sqrt_3, 0.5)
        }
    );
}

#[test]
fn cpp_pline_seg_arc_line_two_intersects_second_line_direction_order() {
    // Source-aligned with old C++ `intrPlineSegs` two-intersect arc-line path:
    // output order follows second segment (line) direction.
    let half_sqrt_3 = 3.0_f64.sqrt() / 2.0;
    let arc_start = PlineVertex::new(1.0, 1.0, 1.0);
    let arc_end = PlineVertex::new(3.0, 1.0, 0.0);
    let line_start = PlineVertex::new(4.0, 0.5, 0.0);
    let line_end = PlineVertex::new(0.0, 0.5, 0.0);

    let result = pline_seg_intr(arc_start, arc_end, line_start, line_end, 1e-5);
    assert_case_eq!(
        result,
        TwoIntersects {
            point1: Vector2::new(2.0 + half_sqrt_3, 0.5),
            point2: Vector2::new(2.0 - half_sqrt_3, 0.5)
        }
    );
}
