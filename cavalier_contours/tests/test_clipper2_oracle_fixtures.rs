mod test_utils;

use std::fmt::Write as _;
use std::{env, fs, path::Path};

use cavalier_contours::pline_closed;
use cavalier_contours::polyline::{BooleanOp, Polyline};
use test_utils::{
    BooleanFixtureInput, BooleanFixtureOptions, ComparisonMode, ExpectedFixtureData, FixtureCase,
    FixtureMetadata, FixtureOperation, FixtureOperationKind, FixtureProvenance, FixtureTolerance,
    GeometryModel, OffsetFixtureInput, OffsetFixtureOptions, PlineProperties,
    PropertiesFixtureInput, PropertyExpectationOptions, UsageLabel, fixture_metadata, run_fixture,
};

const CLIPPER2_REPO: &str = "Clipper2";
const CLIPPER2_COMMIT: &str = "f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd";
const CLIPPER2_LICENSE: &str = "Boost Software License 1.0";

fn clipper2_provenance(source_path: &'static str) -> FixtureProvenance {
    FixtureProvenance {
        source_repo: CLIPPER2_REPO,
        source_commit: CLIPPER2_COMMIT,
        source_path,
        license: CLIPPER2_LICENSE,
        usage_label: UsageLabel::OracleComparable,
    }
}

fn oracle_property_options() -> PropertyExpectationOptions {
    PropertyExpectationOptions {
        compare_abs_area: true,
        ..PropertyExpectationOptions::default()
    }
}

fn clipper2_polytree_intersection_square_overlap() -> FixtureCase {
    FixtureCase::new(
        "clipper2-polytree-intersection-square-overlap",
        clipper2_provenance("CPP/Tests/TestPolytreeIntersection.cpp"),
        GeometryModel::PolygonPath,
        FixtureOperation::Boolean(BooleanFixtureInput {
            subject: pline_closed![
                (0.0, 0.0, 0.0),
                (0.0, 5.0, 0.0),
                (5.0, 5.0, 0.0),
                (5.0, 0.0, 0.0),
            ],
            clip: pline_closed![
                (1.0, 1.0, 0.0),
                (1.0, 6.0, 0.0),
                (6.0, 6.0, 0.0),
                (6.0, 1.0, 0.0),
            ],
            op: BooleanOp::And,
            options: BooleanFixtureOptions::default(),
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Boolean {
            pos_result: vec![PlineProperties::new(
                4,
                16.0,
                16.0,
                1.0,
                1.0,
                5.0,
                5.0,
                vec![],
            )],
            neg_result: vec![],
            options: oracle_property_options(),
        },
    )
}

fn clipper2_offset_007_collapsed_square() -> FixtureCase {
    FixtureCase::new(
        "clipper2-offset-007-collapsed-square",
        clipper2_provenance("CPP/Tests/TestOffsets.cpp"),
        GeometryModel::PolygonPath,
        FixtureOperation::Offset(OffsetFixtureInput {
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (100.0, 0.0, 0.0),
                (100.0, 100.0, 0.0),
                (0.0, 100.0, 0.0),
            ],
            // Clipper2 source: InflatePaths(subject, -50, JoinType::Miter,
            // EndType::Polygon). Current Rust's positive offset direction is
            // the comparable interior/collapse direction for this CCW polygon.
            // No arc-to-polygon approximation is involved.
            offset: 50.0,
            options: OffsetFixtureOptions::default(),
        }),
        ComparisonMode::ApproximateParity,
        FixtureTolerance::default(),
        ExpectedFixtureData::Offset {
            result: vec![],
            options: oracle_property_options(),
        },
    )
}

fn clipper2_polygons_017_intersection_evenodd() -> FixtureCase {
    FixtureCase::new(
        "clipper2-polygons-017-intersection-evenodd",
        clipper2_provenance("Tests/Polygons.txt"),
        GeometryModel::PolygonPath,
        FixtureOperation::Boolean(BooleanFixtureInput {
            subject: Polyline::new(),
            clip: Polyline::new(),
            op: BooleanOp::And,
            options: BooleanFixtureOptions::default(),
        }),
        ComparisonMode::NotComparable,
        FixtureTolerance::default(),
        ExpectedFixtureData::MetadataOnly {
            reason: "Clipper2 Polygons.txt case 17 records SOL_AREA=14779 and SOL_COUNT=1, but Phase 5 keeps broad even-odd text fixtures metadata-only until a precise two-polyline Rust mapping is manually verified",
        },
    )
}

fn clipper2_offsets_001_round_polygon() -> FixtureCase {
    FixtureCase::new(
        "clipper2-offsets-001-round-polygon",
        clipper2_provenance("Tests/Offsets.txt"),
        GeometryModel::PolygonPath,
        FixtureOperation::Offset(OffsetFixtureInput {
            input: Polyline::new(),
            offset: 1.0,
            options: OffsetFixtureOptions::default(),
        }),
        ComparisonMode::NotComparable,
        FixtureTolerance::default(),
        ExpectedFixtureData::MetadataOnly {
            reason: "Clipper2 Offsets.txt case 1 is loaded with JoinType::Round, EndType::Polygon, and Execute(1), but stored SOL_AREA and SOL_COUNT are skipped in TestOffsets.cpp",
        },
    )
}

fn clipper2_open_lines_suite() -> FixtureCase {
    FixtureCase::new(
        "clipper2-open-lines-suite",
        clipper2_provenance("Tests/Lines.txt"),
        GeometryModel::PolygonPath,
        FixtureOperation::Boolean(BooleanFixtureInput {
            subject: Polyline::new(),
            clip: Polyline::new(),
            op: BooleanOp::And,
            options: BooleanFixtureOptions::default(),
        }),
        ComparisonMode::NotComparable,
        FixtureTolerance::default(),
        ExpectedFixtureData::MetadataOnly {
            reason: "Clipper2 line/open-path clipping does not map to the current closed area polyline fixture operation",
        },
    )
}

fn clipper2_triangulation_suite() -> FixtureCase {
    FixtureCase::new(
        "clipper2-triangulation-suite",
        clipper2_provenance("CPP/Examples/Triangulation"),
        GeometryModel::PolygonPath,
        FixtureOperation::Properties(PropertiesFixtureInput {
            input: Polyline::new(),
        }),
        ComparisonMode::NotComparable,
        FixtureTolerance::default(),
        ExpectedFixtureData::MetadataOnly {
            reason: "triangulation is explicitly deferred by the absorption roadmap",
        },
    )
}

fn executable_clipper2_fixtures() -> Vec<FixtureCase> {
    vec![
        clipper2_polytree_intersection_square_overlap(),
        clipper2_offset_007_collapsed_square(),
    ]
}

fn metadata_only_clipper2_records() -> Vec<FixtureCase> {
    vec![
        clipper2_polygons_017_intersection_evenodd(),
        clipper2_offsets_001_round_polygon(),
        clipper2_open_lines_suite(),
        clipper2_triangulation_suite(),
    ]
}

fn oracle_report_rows() -> Vec<(FixtureCase, &'static str)> {
    executable_clipper2_fixtures()
        .into_iter()
        .map(|fixture| (fixture, "pass"))
        .chain(metadata_only_clipper2_records().into_iter().map(|fixture| {
            let status = match fixture.id {
                "clipper2-triangulation-suite" => "deferred",
                "clipper2-polygons-017-intersection-evenodd"
                | "clipper2-offsets-001-round-polygon" => "deferred",
                _ => "not comparable",
            };
            (fixture, status)
        }))
        .collect()
}

#[test]
fn clipper2_executable_oracle_fixtures_run_through_fixture_harness() {
    for fixture in executable_clipper2_fixtures() {
        let summary = run_fixture(&fixture);
        assert!(summary.executed, "fixture {} should execute", fixture.id);
        assert_eq!(summary.metadata.source_repo, CLIPPER2_REPO);
        assert_eq!(fixture.provenance.license, CLIPPER2_LICENSE);
    }
}

#[test]
fn clipper2_metadata_only_records_do_not_execute() {
    for fixture in metadata_only_clipper2_records() {
        let summary = run_fixture(&fixture);
        assert!(
            !summary.executed,
            "metadata fixture {} should not execute",
            fixture.id
        );
        assert_eq!(summary.metadata.usage_label, UsageLabel::OracleComparable);
    }
}

#[test]
fn clipper2_oracle_report_is_written_when_requested() {
    if env::var("CAVC_CLIPPER2_ORACLE_REPORT").as_deref() != Ok("1") {
        return;
    }

    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("core crate should have a workspace parent");
    let report_dir = workspace_root.join("target").join("clipper2-oracle");
    let report_path = report_dir.join("clipper2-oracle-report.md");
    fs::create_dir_all(&report_dir).expect("failed to create target/clipper2-oracle");

    let mut report = String::new();
    writeln!(&mut report, "# Clipper2 Oracle Report").unwrap();
    writeln!(&mut report).unwrap();
    writeln!(
        &mut report,
        "Generated by `CAVC_CLIPPER2_ORACLE_REPORT=1 cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture`."
    )
    .unwrap();
    writeln!(
        &mut report,
        "Oracle results are Phase 6 gap-ranking evidence, not production behavior."
    )
    .unwrap();
    writeln!(
        &mut report,
        "Status vocabulary: pass, gap, not comparable, deferred."
    )
    .unwrap();
    writeln!(&mut report).unwrap();
    writeln!(
        &mut report,
        "| Fixture ID | Operation | Source path | Comparison | Status |"
    )
    .unwrap();
    writeln!(
        &mut report,
        "|------------|-----------|-------------|------------|--------|"
    )
    .unwrap();

    for (fixture, status) in oracle_report_rows() {
        if fixture.is_executable() {
            let summary = run_fixture(&fixture);
            assert!(summary.executed, "fixture {} should execute", fixture.id);
        }

        writeln!(
            &mut report,
            "| `{}` | `{:?}` | `{}` | `{:?}` | {} |",
            fixture.id,
            fixture.operation.kind(),
            fixture.provenance.source_path,
            fixture.comparison,
            status
        )
        .unwrap();
    }

    fs::write(&report_path, report).expect("failed to write Clipper2 oracle report");
    assert!(
        fs::metadata(&report_path).is_ok(),
        "expected oracle report at {}",
        report_path.display()
    );
}

#[test]
fn clipper2_fixture_metadata_is_observable() {
    let executable_fixtures = executable_clipper2_fixtures();
    let executable_metadata = fixture_metadata(&executable_fixtures);

    assert_metadata(
        &executable_metadata,
        "clipper2-polytree-intersection-square-overlap",
        "CPP/Tests/TestPolytreeIntersection.cpp",
        ComparisonMode::ApproximateParity,
        FixtureOperationKind::Boolean,
        true,
    );
    assert_metadata(
        &executable_metadata,
        "clipper2-offset-007-collapsed-square",
        "CPP/Tests/TestOffsets.cpp",
        ComparisonMode::ApproximateParity,
        FixtureOperationKind::Offset,
        true,
    );

    let metadata_records = metadata_only_clipper2_records();
    let metadata = fixture_metadata(&metadata_records);

    assert_metadata(
        &metadata,
        "clipper2-polygons-017-intersection-evenodd",
        "Tests/Polygons.txt",
        ComparisonMode::NotComparable,
        FixtureOperationKind::Boolean,
        false,
    );
    assert_metadata(
        &metadata,
        "clipper2-offsets-001-round-polygon",
        "Tests/Offsets.txt",
        ComparisonMode::NotComparable,
        FixtureOperationKind::Offset,
        false,
    );
    assert_metadata(
        &metadata,
        "clipper2-open-lines-suite",
        "Tests/Lines.txt",
        ComparisonMode::NotComparable,
        FixtureOperationKind::Boolean,
        false,
    );
    assert_metadata(
        &metadata,
        "clipper2-triangulation-suite",
        "CPP/Examples/Triangulation",
        ComparisonMode::NotComparable,
        FixtureOperationKind::Properties,
        false,
    );
}

fn assert_metadata(
    metadata: &[FixtureMetadata],
    id: &'static str,
    source_path: &'static str,
    comparison: ComparisonMode,
    operation: FixtureOperationKind,
    executable: bool,
) {
    let actual = metadata
        .iter()
        .find(|item| item.id == id)
        .unwrap_or_else(|| panic!("missing fixture metadata for {id}"));

    assert_eq!(actual.source_repo, CLIPPER2_REPO);
    assert_eq!(actual.source_path, source_path);
    assert_eq!(actual.usage_label, UsageLabel::OracleComparable);
    assert_eq!(actual.comparison, comparison);
    assert_eq!(actual.operation, operation);
    assert_eq!(actual.executable, executable);
}
