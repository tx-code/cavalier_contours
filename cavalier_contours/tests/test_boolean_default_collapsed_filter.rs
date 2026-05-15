mod test_utils;

use cavalier_contours::polyline::PlineSource;
use cavalier_contours::{pline_closed_userdata, polyline::BooleanOp};
use test_utils::{PlineProperties, create_property_set, property_sets_match_abs_a};

fn assert_default_boolean_case(
    subject: &cavalier_contours::polyline::Polyline<f64>,
    clip: &cavalier_contours::polyline::Polyline<f64>,
    op: BooleanOp,
    expected_pos: &[PlineProperties],
    expected_neg: &[PlineProperties],
) {
    let result = subject.boolean(clip, op);
    let pos = create_property_set(result.pos_plines.iter().map(|r| &r.pline), false);
    let neg = create_property_set(result.neg_plines.iter().map(|r| &r.pline), false);

    assert!(
        property_sets_match_abs_a(&pos, expected_pos),
        "default boolean pos mismatch for op={op:?}"
    );
    assert!(
        property_sets_match_abs_a(&neg, expected_neg),
        "default boolean neg mismatch for op={op:?}"
    );
}

#[test]
fn default_boolean_filters_tiny_collapsed_loop_reported5_modified_not() {
    let subject = pline_closed_userdata![
        [4],
        (10.2548112191951, 4.4473618027979, 0.0),
        (5.8085712191951, 8.0043518027979, 1.0),
        (5.0152087808049, 7.0126481972021, 0.0),
        (9.4614487808049, 3.4556581972021, 1.0)
    ];
    let clip = pline_closed_userdata![
        [117],
        (5.80888046975325, 8.00410413311421, -0.2276528214202),
        (5.09501, 9.4907900625929, 1.0),
        (3.82501, 9.4907899374071, 0.2276528214202),
        (5.0148600112639, 7.0129274139501, 1.0)
    ];

    assert_default_boolean_case(
        &subject,
        &clip,
        BooleanOp::Not,
        &[PlineProperties::new(
            5,
            -7.231366678543611,
            15.377700338744262,
            5.0152087808049,
            3.31650999999997,
            10.49313000000003,
            8.0043518027979,
            vec![4, 117],
        )],
        &[],
    );
}

#[test]
fn default_boolean_filters_tiny_collapsed_loop_debug_stitching_xor() {
    let subject = pline_closed_userdata![
        [4],
        (71.44735180279787, 41.015208780804905, 0.0),
        (75.00434180279787, 45.46144878080491, 1.0),
        (74.01263819720212, 46.2548112191951, 0.0),
        (70.45564819720212, 41.808571219195095, -0.22759115259754015),
        (68.96920993740713, 41.095009999999995, 0.9999999999999999),
        (68.96921006259288, 39.82501, 0.22765282142017604)
    ];
    let clip = pline_closed_userdata![
        [117],
        (62.570000992309986, 39.82500000000078, 0.0),
        (68.96921099231, 39.82501000000077, 1.0),
        (68.96920900769001, 41.09500999999922, 0.0),
        (62.569999007690015, 41.094999999999224, 1.0)
    ];

    assert_default_boolean_case(
        &subject,
        &clip,
        BooleanOp::Xor,
        &[
            PlineProperties::new(
                6,
                -10.119464484796188,
                19.925886964359684,
                68.96920900769001,
                39.82501000984475,
                75.14349,
                46.49313000000001,
                vec![4, 117],
            ),
            PlineProperties::new(
                4,
                8.12699670000984,
                16.788242670074666,
                61.935,
                39.825,
                68.96921099231,
                41.09500999999922,
                vec![4, 117],
            ),
        ],
        &[],
    );
}

#[test]
fn default_boolean_filters_tiny_collapsed_loop_opposite_arc_arc_or() {
    let subject = pline_closed_userdata![
        [4],
        (-188.500000000023, -166.831646988729, 0.0),
        (-188.5, -195.881478300073, 0.0),
        (-189.0, -196.91384910249, 0.553407781718062),
        (-170.999999999999, -225.631646989572, -0.553407781718061),
        (-153.0, -196.91384910249, -0.553407781718095),
        (-153.5, -195.881478300072, 0.0),
        (-153.5, -166.831646988778, 0.0),
        (-153.5, -166.820646988779, 0.0),
        (-188.500000000023, -166.820646988729, 0.0)
    ];
    let clip = pline_closed_userdata![
        [117],
        (412.0, -246.331646989572, 0.0),
        (412.0, -156.831646989572, 0.0),
        (319.0, -156.831646989571, 0.0),
        (319.0, -193.831646989571, 0.0),
        (317.0, -193.831646989571, 0.0),
        (317.0, -156.831646989572, 0.0),
        (-153.5, -156.831646989572, 0.0),
        (-153.5, -195.881478300072, 0.0),
        (-153.0, -196.91384910249, -0.553407781718061),
        (-171.0, -225.631646989571, -0.553407781718061),
        (-189.0, -196.91384910249, 0.0),
        (-188.5, -195.881478300073, 0.0),
        (-188.5, -156.831646989571, 0.0),
        (-498.0, -156.831646989571, 0.0),
        (-498.0, -193.831646989572, 0.0),
        (-448.0, -193.831646989572, 0.0),
        (-448.0, -228.831646989571, 0.0),
        (-538.0, -228.831646989571, 0.0),
        (-538.0, -193.831646989572, 0.0),
        (-500.0, -193.831646989572, 0.0),
        (-500.0, -156.831646989572, 0.0),
        (-618.0, -156.831646989572, 0.0),
        (-618.0, -246.331646989572, 0.0)
    ];

    assert_default_boolean_case(
        &subject,
        &clip,
        BooleanOp::Or,
        &[PlineProperties::new(
            20,
            88537.38500002908,
            2652.977999998363,
            -618.0,
            -246.331646989572,
            412.0,
            -156.831646989571,
            vec![4, 117],
        )],
        &[
            PlineProperties::new(
                2,
                448.72925543646033,
                80.87292554364588,
                -173.0,
                -225.631646989571,
                -151.0,
                -196.91384910249,
                vec![4, 117],
            ),
            PlineProperties::new(
                2,
                0.25699795696345973,
                2.515629079078068,
                -153.60334994589547,
                -196.91384910249,
                -153.0,
                -195.881478300072,
                vec![4, 117],
            ),
        ],
    );
}
