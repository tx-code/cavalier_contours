mod test_utils;

use cavalier_contours::pline_closed;
use cavalier_contours::polyline::{BooleanOp, PlineSource, Polyline};
use test_utils::{PlineProperties, create_property_set, property_sets_match_abs_a};

fn issue_38_inputs() -> (Polyline<f64>, Polyline<f64>) {
    let subject = pline_closed![
        (233.5, 0.0, 0.0),
        (222.0, 233.5, 0.0),
        (-233.5, 233.5, 0.0),
        (-233.5, 0.0, 0.0)
    ];
    let clip = pline_closed![
        (222.0, 233.5, 0.0),
        (232.934, 11.486, 0.4),
        (-0.283, 233.5, 0.0)
    ];
    (subject, clip)
}

fn assert_issue_38_boolean_case(
    op: BooleanOp,
    expected_pos: &[PlineProperties],
    expected_neg: &[PlineProperties],
) {
    let (subject, clip) = issue_38_inputs();
    let result = subject.boolean(&clip, op);

    let pos = create_property_set(result.pos_plines.iter().map(|r| &r.pline), false);
    let neg = create_property_set(result.neg_plines.iter().map(|r| &r.pline), false);

    assert!(
        property_sets_match_abs_a(&pos, expected_pos),
        "issue #38 pos mismatch for op={op:?}"
    );
    assert!(
        property_sets_match_abs_a(&neg, expected_neg),
        "issue #38 neg mismatch for op={op:?}"
    );
}

#[test]
fn issue_38_cut_not_worked_expected_boolean_outputs() {
    // Repro from upstream issue #38:
    // https://github.com/jbuckmccready/cavalier_contours/issues/38
    assert_issue_38_boolean_case(
        BooleanOp::Or,
        &[PlineProperties::new(
            4,
            -107701.875,
            1389.7830190582713,
            -233.5,
            0.0,
            233.5,
            233.5,
            vec![],
        )],
        &[],
    );

    assert_issue_38_boolean_case(
        BooleanOp::And,
        &[PlineProperties::new(
            2,
            0.0,
            444.566,
            -0.283,
            233.5,
            222.0,
            233.5,
            vec![],
        )],
        &[],
    );

    assert_issue_38_boolean_case(BooleanOp::Not, &[], &[]);
    assert_issue_38_boolean_case(BooleanOp::Xor, &[], &[]);
}
