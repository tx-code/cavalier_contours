use cavalier_contours::polyline::{BooleanOp, PlineContainsResult, Polyline};

use super::PlineProperties;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageLabel {
    ForkOwnedChangeable,
    HistoricalReference,
    ReferenceOnly,
    OracleComparable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureProvenance {
    pub source_repo: &'static str,
    pub source_commit: &'static str,
    pub source_path: &'static str,
    pub license: &'static str,
    pub usage_label: UsageLabel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryModel {
    BulgeArcPolyline,
    PolygonPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonMode {
    ExactParity,
    ApproximateParity,
    IntentionalDivergence,
    NotComparable,
    Gap,
}

impl ComparisonMode {
    pub fn is_executable(self) -> bool {
        matches!(self, Self::ExactParity | Self::ApproximateParity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureOperationKind {
    Offset,
    Boolean,
    ContainsProperties,
}

#[derive(Debug, Clone)]
pub enum FixtureOperation {
    Offset(OffsetFixtureInput),
    Boolean(BooleanFixtureInput),
    ContainsProperties(ContainsPropertiesFixtureInput),
}

impl FixtureOperation {
    pub fn kind(&self) -> FixtureOperationKind {
        match self {
            Self::Offset(_) => FixtureOperationKind::Offset,
            Self::Boolean(_) => FixtureOperationKind::Boolean,
            Self::ContainsProperties(_) => FixtureOperationKind::ContainsProperties,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OffsetFixtureInput {
    pub input: Polyline<f64>,
    pub offset: f64,
    pub options: OffsetFixtureOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OffsetFixtureOptions {
    pub handle_self_intersects: bool,
}

#[derive(Debug, Clone)]
pub struct BooleanFixtureInput {
    pub subject: Polyline<f64>,
    pub clip: Polyline<f64>,
    pub op: BooleanOp,
    pub options: BooleanFixtureOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BooleanFixtureOptions {
    pub pos_equal_eps: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ContainsPropertiesFixtureInput {
    pub subject: Polyline<f64>,
    pub clip: Polyline<f64>,
    pub options: ContainsPropertiesFixtureOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ContainsPropertiesFixtureOptions {
    pub pos_equal_eps: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct FixtureTolerance {
    pub property_eps: f64,
    pub position_eps: f64,
    pub remove_redundant_eps: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PropertyExpectationOptions {
    pub compare_orientation: bool,
    pub compare_open_closed: bool,
    pub compare_repeat_vertices: bool,
    pub compare_user_data: bool,
    pub compare_abs_area: bool,
}

#[derive(Debug, Clone)]
pub enum ExpectedFixtureData {
    Offset {
        result: Vec<PlineProperties>,
        options: PropertyExpectationOptions,
    },
    Boolean {
        pos_result: Vec<PlineProperties>,
        neg_result: Vec<PlineProperties>,
        options: PropertyExpectationOptions,
    },
    ContainsProperties {
        contains: PlineContainsResult,
        subject_properties: PlineProperties,
        clip_properties: PlineProperties,
        options: PropertyExpectationOptions,
    },
    MetadataOnly {
        reason: &'static str,
    },
}

impl ExpectedFixtureData {
    pub fn is_metadata_only(&self) -> bool {
        matches!(self, Self::MetadataOnly { .. })
    }
}

#[derive(Debug, Clone)]
pub struct FixtureCase {
    pub id: &'static str,
    pub provenance: FixtureProvenance,
    pub geometry_model: GeometryModel,
    pub operation: FixtureOperation,
    pub comparison: ComparisonMode,
    pub tolerance: FixtureTolerance,
    pub expected: ExpectedFixtureData,
}

impl FixtureCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &'static str,
        provenance: FixtureProvenance,
        geometry_model: GeometryModel,
        operation: FixtureOperation,
        comparison: ComparisonMode,
        tolerance: FixtureTolerance,
        expected: ExpectedFixtureData,
    ) -> Self {
        assert!(!id.trim().is_empty(), "fixture id must not be empty");
        assert!(
            !provenance.source_repo.trim().is_empty()
                && !provenance.source_commit.trim().is_empty()
                && !provenance.source_path.trim().is_empty()
                && !provenance.license.trim().is_empty(),
            "fixture provenance must include repo, commit, path, and license",
        );
        assert!(
            !comparison.is_executable() || !expected.is_metadata_only(),
            "executable fixtures must include operation-specific expected data",
        );

        Self {
            id,
            provenance,
            geometry_model,
            operation,
            comparison,
            tolerance,
            expected,
        }
    }

    pub fn is_executable(&self) -> bool {
        self.comparison.is_executable() && !self.expected.is_metadata_only()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureMetadata {
    pub id: &'static str,
    pub source_repo: &'static str,
    pub source_path: &'static str,
    pub usage_label: UsageLabel,
    pub comparison: ComparisonMode,
    pub operation: FixtureOperationKind,
    pub executable: bool,
}

impl From<&FixtureCase> for FixtureMetadata {
    fn from(fixture: &FixtureCase) -> Self {
        Self {
            id: fixture.id,
            source_repo: fixture.provenance.source_repo,
            source_path: fixture.provenance.source_path,
            usage_label: fixture.provenance.usage_label,
            comparison: fixture.comparison,
            operation: fixture.operation.kind(),
            executable: fixture.is_executable(),
        }
    }
}

pub fn collect_fixture_metadata(fixtures: &[FixtureCase]) -> Vec<FixtureMetadata> {
    fixtures.iter().map(FixtureMetadata::from).collect()
}
