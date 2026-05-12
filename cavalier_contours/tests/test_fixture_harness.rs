mod test_utils;

use cavalier_contours::polyline::{BooleanOp, PlineContainsResult, Polyline};
use cavalier_contours::{pline_closed, pline_closed_userdata};
use test_utils::{
    BooleanFixtureInput, BooleanFixtureOptions, ComparisonMode, ContainsPropertiesFixtureInput,
    ContainsPropertiesFixtureOptions, ExpectedFixtureData, FixtureCase, FixtureOperation,
    FixtureProvenance, FixtureTolerance, GeometryModel, OffsetFixtureInput, OffsetFixtureOptions,
    PlineProperties, PropertiesFixtureInput, PropertyExpectationOptions, UsageLabel,
    fixture_metadata, run_fixture,
};

const CURRENT_RUST_SEED_COMMIT: &str = "a6b56ac";

fn current_rust_provenance(source_path: &'static str) -> FixtureProvenance {
    FixtureProvenance {
        source_repo: "cavalier_contours",
        source_commit: CURRENT_RUST_SEED_COMMIT,
        source_path,
        license: "MIT OR Apache-2.0",
        usage_label: UsageLabel::ForkOwnedChangeable,
    }
}

fn property_options() -> PropertyExpectationOptions {
    PropertyExpectationOptions {
        compare_user_data: true,
        compare_repeat_vertices: true,
        ..Default::default()
    }
}

fn offset_seed() -> FixtureCase {
    FixtureCase::new(
        "current-rust-offset-rectangle-inward",
        current_rust_provenance("cavalier_contours/tests/test_pline_parallel_offset.rs"),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Offset(OffsetFixtureInput {
            input: pline_closed_userdata![
                [4],
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
            ],
            offset: 2.0,
            options: OffsetFixtureOptions {
                handle_self_intersects: false,
            },
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Offset {
            result: vec![PlineProperties::new(
                4,
                96.0,
                44.0,
                2.0,
                2.0,
                18.0,
                8.0,
                vec![4],
            )],
            options: property_options(),
        },
    )
}

fn boolean_seed() -> FixtureCase {
    FixtureCase::new(
        "current-rust-boolean-disjoint-or",
        current_rust_provenance("cavalier_contours/tests/test_pline_boolean.rs"),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Boolean(BooleanFixtureInput {
            subject: pline_closed_userdata![
                [4],
                (0.0, 0.0, 0.0),
                (10.0, 0.0, 0.0),
                (10.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
            ],
            clip: pline_closed_userdata![
                [117],
                (20.0, 0.0, 0.0),
                (30.0, 0.0, 0.0),
                (30.0, 10.0, 0.0),
                (20.0, 10.0, 0.0),
            ],
            op: BooleanOp::Or,
            options: BooleanFixtureOptions::default(),
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Boolean {
            pos_result: vec![
                PlineProperties::new(4, 100.0, 40.0, 0.0, 0.0, 10.0, 10.0, vec![4]),
                PlineProperties::new(4, 100.0, 40.0, 20.0, 0.0, 30.0, 10.0, vec![117]),
            ],
            neg_result: vec![],
            options: property_options(),
        },
    )
}

fn contains_properties_seed() -> FixtureCase {
    FixtureCase::new(
        "current-rust-contains-rectangle-circle",
        current_rust_provenance("cavalier_contours/tests/test_pline_contains.rs"),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::ContainsProperties(ContainsPropertiesFixtureInput {
            subject: pline_closed![
                (-2.0, -2.0, 0.0),
                (2.0, -2.0, 0.0),
                (2.0, 2.0, 0.0),
                (-2.0, 2.0, 0.0),
            ],
            clip: pline_closed![(-1.0, 0.0, 1.0), (1.0, 0.0, 1.0)],
            options: ContainsPropertiesFixtureOptions::default(),
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::ContainsProperties {
            contains: PlineContainsResult::Pline2InsidePline1,
            subject_properties: PlineProperties::new(4, 16.0, 16.0, -2.0, -2.0, 2.0, 2.0, vec![]),
            clip_properties: PlineProperties::new(
                2,
                std::f64::consts::PI,
                2.0 * std::f64::consts::PI,
                -1.0,
                -1.0,
                1.0,
                1.0,
                vec![],
            ),
            options: property_options(),
        },
    )
}

fn properties_seed() -> FixtureCase {
    FixtureCase::new(
        "current-rust-properties-rectangle",
        current_rust_provenance("cavalier_contours/tests/test_pline_properties.rs"),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Properties(PropertiesFixtureInput {
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (4.0, 0.0, 0.0),
                (4.0, 3.0, 0.0),
                (0.0, 3.0, 0.0),
            ],
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Properties {
            result: PlineProperties::new(4, 12.0, 14.0, 0.0, 0.0, 4.0, 3.0, vec![]),
            options: PropertyExpectationOptions::default(),
        },
    )
}

fn metadata_only_gap_seed() -> FixtureCase {
    FixtureCase::new(
        "metadata-only-gap-future-oracle-classification",
        current_rust_provenance("phase-02-metadata-only-gap-seed"),
        GeometryModel::PolygonPath,
        FixtureOperation::Offset(OffsetFixtureInput {
            input: Polyline::new(),
            offset: 0.0,
            options: OffsetFixtureOptions::default(),
        }),
        ComparisonMode::Gap,
        FixtureTolerance::default(),
        ExpectedFixtureData::MetadataOnly {
            reason: "records a future oracle/comparability gap without executing assertions",
        },
    )
}

#[test]
fn current_rust_seed_fixtures_execute_through_runner() {
    for fixture in [
        offset_seed(),
        boolean_seed(),
        contains_properties_seed(),
        properties_seed(),
    ] {
        let summary = run_fixture(&fixture);
        assert!(summary.executed, "fixture {} should execute", fixture.id);
        assert_eq!(
            summary.metadata.usage_label,
            UsageLabel::ForkOwnedChangeable
        );
    }
}

#[test]
fn metadata_only_seed_is_recorded_without_execution() {
    let fixtures = vec![metadata_only_gap_seed()];
    let run_summary = run_fixture(&fixtures[0]);
    let metadata = fixture_metadata(&fixtures);

    assert!(!run_summary.executed);
    assert_eq!(metadata.len(), 1);
    assert_eq!(
        metadata[0].id,
        "metadata-only-gap-future-oracle-classification"
    );
    assert_eq!(metadata[0].comparison, ComparisonMode::Gap);
    assert!(!metadata[0].executable);
}
