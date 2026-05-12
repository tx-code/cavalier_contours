use cavalier_contours::polyline::{
    BooleanResultPline, PlineBooleanOptions, PlineContainsOptions, PlineOffsetOptions, PlineSource,
    Polyline,
};

use super::{
    BooleanFixtureOptions, ContainsPropertiesFixtureOptions, ExpectedFixtureData, FixtureCase,
    FixtureMetadata, FixtureOperation, FixtureTolerance, OffsetFixtureOptions, PlineProperties,
    PropertyExpectationOptions, collect_fixture_metadata, create_property_set_with_eps,
    property_sets_match_with_options,
};

impl Default for FixtureTolerance {
    fn default() -> Self {
        Self {
            property_eps: PlineProperties::PROP_CMP_EPS,
            position_eps: PlineProperties::POS_EQ_EPS,
            remove_redundant_eps: PlineProperties::REMOVE_REDUNDANT_EPS,
        }
    }
}

impl FixtureTolerance {
    fn offset_options(self, options: OffsetFixtureOptions) -> PlineOffsetOptions<'static, f64> {
        PlineOffsetOptions {
            aabb_index: None,
            handle_self_intersects: options.handle_self_intersects,
            pos_equal_eps: self.position_eps,
            slice_join_eps: self.remove_redundant_eps,
            offset_dist_eps: self.remove_redundant_eps,
        }
    }

    fn boolean_options(self, options: BooleanFixtureOptions) -> PlineBooleanOptions<'static, f64> {
        PlineBooleanOptions {
            pline1_aabb_index: None,
            pos_equal_eps: options.pos_equal_eps.unwrap_or(self.position_eps),
            collapsed_area_eps: None,
        }
    }

    fn contains_options(
        self,
        options: ContainsPropertiesFixtureOptions,
    ) -> PlineContainsOptions<'static, f64> {
        PlineContainsOptions {
            pline1_aabb_index: None,
            pos_equal_eps: options.pos_equal_eps.unwrap_or(self.position_eps),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureRunSummary {
    pub metadata: FixtureMetadata,
    pub executed: bool,
}

pub fn fixture_metadata(fixtures: &[FixtureCase]) -> Vec<FixtureMetadata> {
    collect_fixture_metadata(fixtures)
}

pub fn run_fixture(fixture: &FixtureCase) -> FixtureRunSummary {
    let metadata = FixtureMetadata::from(fixture);
    if !fixture.is_executable() {
        return FixtureRunSummary {
            metadata,
            executed: false,
        };
    }

    match (&fixture.operation, &fixture.expected) {
        (FixtureOperation::Offset(input), ExpectedFixtureData::Offset { result, options }) => {
            let offset_options = fixture.tolerance.offset_options(input.options);
            let actual = input
                .input
                .parallel_offset_opt(input.offset, &offset_options);
            let actual_properties =
                create_fixture_property_set(actual.iter(), false, fixture.tolerance);
            assert_fixture_property_sets_match(
                fixture,
                "offset result",
                &actual_properties,
                result,
                options,
            );
        }
        (
            FixtureOperation::Boolean(input),
            ExpectedFixtureData::Boolean {
                pos_result,
                neg_result,
                options,
            },
        ) => {
            let boolean_options = fixture.tolerance.boolean_options(input.options);
            let actual = input
                .subject
                .boolean_opt(&input.clip, input.op, &boolean_options);
            let actual_pos = boolean_result_property_set(&actual.pos_plines, fixture.tolerance);
            let actual_neg = boolean_result_property_set(&actual.neg_plines, fixture.tolerance);
            assert_fixture_property_sets_match(
                fixture,
                "boolean positive result",
                &actual_pos,
                pos_result,
                options,
            );
            assert_fixture_property_sets_match(
                fixture,
                "boolean negative result",
                &actual_neg,
                neg_result,
                options,
            );
        }
        (
            FixtureOperation::ContainsProperties(input),
            ExpectedFixtureData::ContainsProperties {
                contains,
                subject_properties,
                clip_properties,
                options,
            },
        ) => {
            let contains_options = fixture.tolerance.contains_options(input.options);
            let actual_contains = input.subject.contains_opt(&input.clip, &contains_options);
            assert_eq!(
                &actual_contains,
                contains,
                "fixture `{}` contains result mismatch\nsource: {}@{}:{}\noperation: {:?}\ncomparison: {:?}\ntolerance: {:?}",
                fixture.id,
                fixture.provenance.source_repo,
                fixture.provenance.source_commit,
                fixture.provenance.source_path,
                fixture.operation.kind(),
                fixture.comparison,
                fixture.tolerance,
            );

            let actual_subject = PlineProperties::from_pline_with_eps(
                &input.subject,
                false,
                fixture.tolerance.remove_redundant_eps,
                fixture.tolerance.position_eps,
            );
            let actual_clip = PlineProperties::from_pline_with_eps(
                &input.clip,
                false,
                fixture.tolerance.remove_redundant_eps,
                fixture.tolerance.position_eps,
            );
            assert_fixture_property_sets_match(
                fixture,
                "contains subject properties",
                &[actual_subject],
                std::slice::from_ref(subject_properties),
                options,
            );
            assert_fixture_property_sets_match(
                fixture,
                "contains clip properties",
                &[actual_clip],
                std::slice::from_ref(clip_properties),
                options,
            );
        }
        _ => {
            panic!(
                "fixture `{}` operation and expected data variants do not match: operation={:?}, expected={:?}",
                fixture.id, fixture.operation, fixture.expected
            );
        }
    }

    FixtureRunSummary {
        metadata,
        executed: true,
    }
}

fn boolean_result_property_set(
    polylines: &[BooleanResultPline<Polyline>],
    tolerance: FixtureTolerance,
) -> Vec<PlineProperties> {
    create_fixture_property_set(polylines.iter().map(|p| &p.pline), false, tolerance)
}

fn create_fixture_property_set<'a, I>(
    polylines: I,
    invert_area: bool,
    tolerance: FixtureTolerance,
) -> Vec<PlineProperties>
where
    I: IntoIterator<Item = &'a Polyline>,
{
    create_property_set_with_eps(
        polylines,
        invert_area,
        tolerance.remove_redundant_eps,
        tolerance.position_eps,
    )
}

fn assert_fixture_property_sets_match(
    fixture: &FixtureCase,
    label: &str,
    actual: &[PlineProperties],
    expected: &[PlineProperties],
    options: &PropertyExpectationOptions,
) {
    assert!(
        property_sets_match_with_options(actual, expected, fixture.tolerance.property_eps, options),
        "fixture `{}` {label} properties mismatch\nsource: {}@{}:{}\noperation: {:?}\ncomparison: {:?}\ntolerance: {:?}\nactual: {actual:#?}\nexpected: {expected:#?}",
        fixture.id,
        fixture.provenance.source_repo,
        fixture.provenance.source_commit,
        fixture.provenance.source_path,
        fixture.operation.kind(),
        fixture.comparison,
        fixture.tolerance,
    );
}
