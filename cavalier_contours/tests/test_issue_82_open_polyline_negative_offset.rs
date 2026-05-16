use cavalier_contours::polyline::{PlineOffsetOptions, PlineSource, PlineSourceMut, Polyline};

fn issue_82_input() -> Polyline<f64> {
    let mut pl = Polyline::new();
    pl.add(28.7793, 24.1251, 0.0);
    pl.add(26.6719, 18.6144, 0.0);
    pl.add(27.4604, 13.6769, 0.157308);
    pl.add(28.4408, 11.7648, 0.0);
    pl.add(42.1788, -1.97424, 0.198723);
    pl.add(44.6542, -3.0, 0.0);
    pl.add(49.6542, -3.0, 0.0);
    pl.add(54.7638, -0.0499998, 0.0);
    pl
}

#[test]
fn issue_82_open_polyline_negative_offset_not_empty() {
    // Repro from upstream issue #82:
    // https://github.com/jbuckmccready/cavalier_contours/issues/82
    let input = issue_82_input();
    assert!(!input.is_closed());

    let options = PlineOffsetOptions::default();
    let positive = input.parallel_offset_opt(0.005, &options);
    let negative = input.parallel_offset_opt(-0.005, &options);

    assert!(
        !positive.is_empty(),
        "expected positive offset to produce at least one result"
    );
    assert!(
        !negative.is_empty(),
        "expected negative offset to produce at least one result"
    );
}
