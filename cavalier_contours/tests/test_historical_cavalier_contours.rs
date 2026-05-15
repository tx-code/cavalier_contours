mod test_utils;

use cavalier_contours::pline_closed;
use cavalier_contours::polyline::{BooleanOp, Polyline};
use test_utils::{
    BooleanFixtureInput, BooleanFixtureOptions, ComparisonMode, ExpectedFixtureData, FixtureCase,
    FixtureOperation, FixtureProvenance, FixtureTolerance, GeometryModel, OffsetFixtureInput,
    OffsetFixtureOptions, PlineProperties, PropertiesFixtureInput, PropertyExpectationOptions,
    UsageLabel, fixture_metadata, run_fixture,
};
use test_utils::{FixtureMetadata, FixtureOperationKind};

const OLD_CPP_REPO: &str = "CavalierContours";
const OLD_CPP_COMMIT: &str = "31a012947aa2e7e9474e2ec90502825afe8b99a4";
const OLD_CPP_LICENSE: &str = "MIT";

fn old_cpp_provenance(source_path: &'static str, usage_label: UsageLabel) -> FixtureProvenance {
    FixtureProvenance {
        source_repo: OLD_CPP_REPO,
        source_commit: OLD_CPP_COMMIT,
        source_path,
        license: OLD_CPP_LICENSE,
        usage_label,
    }
}

fn historical_property_options() -> PropertyExpectationOptions {
    PropertyExpectationOptions::default()
}

fn historical_geometry_parity_options() -> PropertyExpectationOptions {
    PropertyExpectationOptions {
        compare_vertex_count: false,
        compare_abs_area: true,
        ..PropertyExpectationOptions::default()
    }
}

fn historical_cpp_offset_closed_rectangle_inward() -> FixtureCase {
    FixtureCase::new(
        "historical-cpp-offset-closed-rectangle-inward",
        old_cpp_provenance(
            "tests/tests/TEST_cavc_parallel_offset.cpp",
            UsageLabel::TranslatedFixtureCandidate,
        ),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Offset(OffsetFixtureInput {
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
            ],
            offset: 2.0,
            options: OffsetFixtureOptions::default(),
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
                vec![],
            )],
            options: historical_property_options(),
        },
    )
}

fn historical_cpp_offset_collapsed_rectangle() -> FixtureCase {
    FixtureCase::new(
        "historical-cpp-offset-collapsed-rectangle",
        old_cpp_provenance(
            "tests/tests/TEST_cavc_parallel_offset.cpp",
            UsageLabel::TranslatedFixtureCandidate,
        ),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Offset(OffsetFixtureInput {
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (120.0, 0.0, 0.0),
                (120.0, 40.0, 0.0),
                (0.0, 40.0, 0.0),
            ],
            offset: 30.0,
            options: OffsetFixtureOptions::default(),
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Offset {
            result: vec![],
            options: historical_property_options(),
        },
    )
}

fn historical_cpp_combine_circle_rectangle_union() -> FixtureCase {
    FixtureCase::new(
        "historical-cpp-combine-circle-rectangle-union",
        old_cpp_provenance(
            "tests/tests/TEST_cavc_combine_plines.cpp",
            UsageLabel::TranslatedFixtureCandidate,
        ),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Boolean(BooleanFixtureInput {
            subject: pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)],
            clip: pline_closed![
                (3.0, -10.0, 0.0),
                (6.0, -10.0, 0.0),
                (6.0, 10.0, 0.0),
                (3.0, 10.0, 0.0),
            ],
            op: BooleanOp::Or,
            options: BooleanFixtureOptions::default(),
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Boolean {
            pos_result: vec![PlineProperties::new(
                10,
                109.15381629282,
                52.324068506275,
                0.0,
                -10.0,
                10.0,
                10.0,
                vec![],
            )],
            neg_result: vec![],
            options: historical_geometry_parity_options(),
        },
    )
}

fn historical_cpp_properties_ccw_circle_x_aligned() -> FixtureCase {
    FixtureCase::new(
        "historical-cpp-properties-ccw-circle-x-aligned",
        old_cpp_provenance(
            "tests/tests/TEST_cavc_pline_function.cpp",
            UsageLabel::TranslatedFixtureCandidate,
        ),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Properties(PropertiesFixtureInput {
            input: pline_closed![(-4.0, 1.0, 1.0), (6.0, 1.0, 1.0)],
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Properties {
            result: PlineProperties::new(
                2,
                std::f64::consts::PI * 25.0,
                2.0 * std::f64::consts::PI * 5.0,
                -4.0,
                -4.0,
                6.0,
                6.0,
                vec![],
            ),
            options: historical_property_options(),
        },
    )
}

fn historical_cpp_c_api_surface_migration_record() -> FixtureCase {
    FixtureCase::new(
        "historical-cpp-c-api-surface-migration-record",
        old_cpp_provenance(
            "c_api_include/cavaliercontours.h",
            UsageLabel::MigrationSensitive,
        ),
        GeometryModel::BulgeArcPolyline,
        FixtureOperation::Properties(PropertiesFixtureInput {
            input: Polyline::new(),
        }),
        ComparisonMode::NotComparable,
        FixtureTolerance::default(),
        ExpectedFixtureData::MetadataOnly {
            reason: "old C API construction/list/offset/combine/property surface is migration-sensitive metadata only in Phase 3",
        },
    )
}

fn historical_cpp_static_spatial_index_query_record() -> FixtureCase {
    FixtureCase::new(
        "historical-cpp-static-spatial-index-query-record",
        old_cpp_provenance(
            "tests/tests/TEST_staticspatialindex.cpp",
            UsageLabel::BenchmarkCandidate,
        ),
        GeometryModel::PolygonPath,
        FixtureOperation::Offset(OffsetFixtureInput {
            input: Polyline::new(),
            offset: 0.0,
            options: OffsetFixtureOptions::default(),
        }),
        ComparisonMode::NotComparable,
        FixtureTolerance::default(),
        ExpectedFixtureData::MetadataOnly {
            reason: "static spatial index query parity is covered by test_cpp_static_spatial_index_parity; this record keeps benchmark-candidate provenance deferred to Phase 4",
        },
    )
}

fn executable_historical_fixtures() -> Vec<FixtureCase> {
    vec![
        historical_cpp_offset_closed_rectangle_inward(),
        historical_cpp_offset_collapsed_rectangle(),
        historical_cpp_combine_circle_rectangle_union(),
        historical_cpp_properties_ccw_circle_x_aligned(),
    ]
}

fn metadata_only_historical_records() -> Vec<FixtureCase> {
    vec![
        historical_cpp_c_api_surface_migration_record(),
        historical_cpp_static_spatial_index_query_record(),
    ]
}

#[test]
fn historical_executable_fixtures_execute_through_runner() {
    for fixture in executable_historical_fixtures() {
        let summary = run_fixture(&fixture);
        assert!(summary.executed, "fixture {} should execute", fixture.id);
        assert_eq!(summary.metadata.source_repo, OLD_CPP_REPO);
    }
}

#[test]
fn historical_metadata_records_do_not_execute() {
    for fixture in metadata_only_historical_records() {
        let summary = run_fixture(&fixture);
        assert!(
            !summary.executed,
            "metadata fixture {} should not execute",
            fixture.id
        );
        assert_eq!(summary.metadata.source_repo, OLD_CPP_REPO);
    }
}

#[test]
fn historical_fixture_metadata_is_observable() {
    let executable_fixtures = executable_historical_fixtures();
    let executable_metadata = fixture_metadata(&executable_fixtures);

    assert_metadata(
        &executable_metadata,
        "historical-cpp-offset-closed-rectangle-inward",
        "tests/tests/TEST_cavc_parallel_offset.cpp",
        UsageLabel::TranslatedFixtureCandidate,
        ComparisonMode::ApproximateParity,
        FixtureOperationKind::Offset,
        true,
    );
    assert_metadata(
        &executable_metadata,
        "historical-cpp-offset-collapsed-rectangle",
        "tests/tests/TEST_cavc_parallel_offset.cpp",
        UsageLabel::TranslatedFixtureCandidate,
        ComparisonMode::ApproximateParity,
        FixtureOperationKind::Offset,
        true,
    );
    assert_metadata(
        &executable_metadata,
        "historical-cpp-combine-circle-rectangle-union",
        "tests/tests/TEST_cavc_combine_plines.cpp",
        UsageLabel::TranslatedFixtureCandidate,
        ComparisonMode::ApproximateParity,
        FixtureOperationKind::Boolean,
        true,
    );
    assert_metadata(
        &executable_metadata,
        "historical-cpp-properties-ccw-circle-x-aligned",
        "tests/tests/TEST_cavc_pline_function.cpp",
        UsageLabel::TranslatedFixtureCandidate,
        ComparisonMode::ApproximateParity,
        FixtureOperationKind::Properties,
        true,
    );

    let metadata_only_records = metadata_only_historical_records();
    let metadata_only = fixture_metadata(&metadata_only_records);

    assert_metadata(
        &metadata_only,
        "historical-cpp-c-api-surface-migration-record",
        "c_api_include/cavaliercontours.h",
        UsageLabel::MigrationSensitive,
        ComparisonMode::NotComparable,
        FixtureOperationKind::Properties,
        false,
    );
    assert_metadata(
        &metadata_only,
        "historical-cpp-static-spatial-index-query-record",
        "tests/tests/TEST_staticspatialindex.cpp",
        UsageLabel::BenchmarkCandidate,
        ComparisonMode::NotComparable,
        FixtureOperationKind::Offset,
        false,
    );
}

fn assert_metadata(
    metadata: &[FixtureMetadata],
    id: &'static str,
    source_path: &'static str,
    usage_label: UsageLabel,
    comparison: ComparisonMode,
    operation: FixtureOperationKind,
    executable: bool,
) {
    let actual = metadata
        .iter()
        .find(|item| item.id == id)
        .unwrap_or_else(|| panic!("missing fixture metadata for {id}"));

    assert_eq!(actual.source_repo, OLD_CPP_REPO);
    assert_eq!(actual.source_path, source_path);
    assert_eq!(actual.usage_label, usage_label);
    assert_eq!(actual.comparison, comparison);
    assert_eq!(actual.operation, operation);
    assert_eq!(actual.executable, executable);
}
