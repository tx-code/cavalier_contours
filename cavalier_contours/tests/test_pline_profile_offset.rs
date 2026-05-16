use cavalier_contours::core::math::Vector2;
use cavalier_contours::polyline::{
    PlineOffsetProfileMode, PlineProfileOffsetError, PlineProfileOffsetOptions, PlineSource,
    PlineSourceMut, Polyline,
};

fn open_l_shape() -> Polyline<f64> {
    let mut pl = Polyline::new();
    pl.add(0.0, 0.0, 0.0);
    pl.add(10.0, 0.0, 0.0);
    pl.add(10.0, 10.0, 0.0);
    pl
}

#[test]
fn profile_offset_linear_mode_open_line_only() {
    let input = open_l_shape();
    let options = PlineProfileOffsetOptions::default();
    let result = input
        .parallel_offset_profile(&[1.0, 2.0, 3.0], &options)
        .expect("expected profile offset to succeed");

    assert_eq!(result.len(), 1);
    let output = &result[0];
    assert!(!output.is_closed());
    assert_eq!(output.vertex_count(), 3);

    let join_expected = Vector2::new(790.0 / 101.0, 180.0 / 101.0);
    assert!(
        output
            .at(0)
            .pos()
            .fuzzy_eq_eps(Vector2::new(0.0, 1.0), 1e-9)
    );
    assert!(output.at(1).pos().fuzzy_eq_eps(join_expected, 1e-9));
    assert!(
        output
            .at(2)
            .pos()
            .fuzzy_eq_eps(Vector2::new(7.0, 10.0), 1e-9)
    );
}

#[test]
fn profile_offset_step_mode_open_line_only() {
    let input = open_l_shape();
    let options = PlineProfileOffsetOptions {
        profile_mode: PlineOffsetProfileMode::StepPerSegment,
        ..Default::default()
    };
    let result = input
        .parallel_offset_profile(&[1.0, 2.0, 3.0], &options)
        .expect("expected profile offset to succeed");

    assert_eq!(result.len(), 1);
    let output = &result[0];
    assert!(!output.is_closed());
    assert_eq!(output.vertex_count(), 3);
    assert!(
        output
            .at(0)
            .pos()
            .fuzzy_eq_eps(Vector2::new(0.0, 1.0), 1e-9)
    );
    assert!(
        output
            .at(1)
            .pos()
            .fuzzy_eq_eps(Vector2::new(8.0, 1.0), 1e-9)
    );
    assert!(
        output
            .at(2)
            .pos()
            .fuzzy_eq_eps(Vector2::new(8.0, 10.0), 1e-9)
    );
}

#[test]
fn profile_offset_rejects_invalid_profile_length() {
    let input = open_l_shape();
    let options = PlineProfileOffsetOptions::default();
    let err = input
        .parallel_offset_profile(&[1.0, 2.0], &options)
        .expect_err("expected invalid profile length error");

    assert_eq!(
        err,
        PlineProfileOffsetError::InvalidProfileLength {
            expected: 3,
            actual: 2
        }
    );
}

#[test]
fn profile_offset_rejects_mixed_signs() {
    let mut input = Polyline::new();
    input.add(0.0, 0.0, 0.0);
    input.add(10.0, 0.0, 0.0);

    let options = PlineProfileOffsetOptions::default();
    let err = input
        .parallel_offset_profile(&[1.0, -1.0], &options)
        .expect_err("expected mixed sign rejection");

    assert_eq!(err, PlineProfileOffsetError::MixedOffsetSigns);
}

#[test]
fn profile_offset_rejects_arc_segments() {
    let mut input = Polyline::new();
    input.add(0.0, 0.0, 1.0);
    input.add(1.0, 0.0, 0.0);

    let options = PlineProfileOffsetOptions::default();
    let err = input
        .parallel_offset_profile(&[1.0, 1.0], &options)
        .expect_err("expected arc unsupported error");

    assert_eq!(
        err,
        PlineProfileOffsetError::ArcSegmentUnsupported { seg_start_index: 0 }
    );
}

#[test]
fn profile_offset_rejects_closed_polyline_for_now() {
    let mut input = Polyline::new_closed();
    input.add(0.0, 0.0, 0.0);
    input.add(10.0, 0.0, 0.0);
    input.add(10.0, 10.0, 0.0);
    input.add(0.0, 10.0, 0.0);

    let options = PlineProfileOffsetOptions::default();
    let err = input
        .parallel_offset_profile(&[1.0, 1.0, 1.0, 1.0], &options)
        .expect_err("expected closed polyline unsupported error");

    assert_eq!(err, PlineProfileOffsetError::ClosedPolylineUnsupported);
}

#[test]
fn profile_offset_rejects_degenerate_segments() {
    let mut input = Polyline::new();
    input.add(0.0, 0.0, 0.0);
    input.add(0.0, 0.0, 0.0);
    input.add(1.0, 0.0, 0.0);

    let options = PlineProfileOffsetOptions::default();
    let err = input
        .parallel_offset_profile(&[1.0, 1.0, 1.0], &options)
        .expect_err("expected degenerate segment error");

    assert_eq!(
        err,
        PlineProfileOffsetError::DegenerateSegment { seg_start_index: 0 }
    );
}
