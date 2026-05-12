use cavalier_contours::{
    core::traits::FuzzyEq,
    polyline::{PlineOrientation, PlineSource, Polyline},
};
use static_aabb2d_index::AABB;

use super::PropertyExpectationOptions;

/// Fuzzy compare AABB values
pub fn aabb_fuzzy_eq_eps(a: &AABB<f64>, b: &AABB<f64>, eps: f64) -> bool {
    a.min_x.fuzzy_eq_eps(b.min_x, eps)
        && a.min_y.fuzzy_eq_eps(b.min_y, eps)
        && a.max_x.fuzzy_eq_eps(b.max_x, eps)
        && a.max_y.fuzzy_eq_eps(b.max_y, eps)
}

/// Holds a set of properties of a polyline for comparison in tests
#[derive(Debug, Clone)]
pub struct PlineProperties {
    pub vertex_count: usize,
    pub area: f64,
    pub path_length: f64,
    pub extents: AABB<f64>,
    pub userdata: Vec<u64>,
    pub is_closed: bool,
    pub orientation: PlineOrientation,
    pub has_repeat_position: bool,
}

impl PlineProperties {
    // positions equal epsilon
    pub const POS_EQ_EPS: f64 = 1e-5;
    // property comparer epsilon
    pub const PROP_CMP_EPS: f64 = 1e-4;
    // epsilon for use of remove_redundant for consistent property compare
    pub const REMOVE_REDUNDANT_EPS: f64 = 1e-4;

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        vertex_count: usize,
        area: f64,
        path_length: f64,
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
        userdata: Vec<u64>,
    ) -> Self {
        Self {
            vertex_count,
            area,
            path_length,
            extents: AABB::new(min_x, min_y, max_x, max_y),
            userdata,
            is_closed: true,
            orientation: if area < 0.0 {
                PlineOrientation::Clockwise
            } else {
                PlineOrientation::CounterClockwise
            },
            has_repeat_position: false,
        }
    }

    pub fn from_pline(pline: &Polyline<f64>, invert_area: bool) -> Self {
        Self::from_pline_with_eps(
            pline,
            invert_area,
            Self::REMOVE_REDUNDANT_EPS,
            Self::POS_EQ_EPS,
        )
    }

    pub fn from_pline_with_eps(
        pline: &Polyline<f64>,
        invert_area: bool,
        remove_redundant_eps: f64,
        pos_equal_eps: f64,
    ) -> Self {
        // remove redundant vertexes for consistent vertex counts
        let rr = pline.remove_redundant(remove_redundant_eps);
        let pline = rr.as_ref().unwrap_or(pline);
        let area = {
            let a = pline.area();
            if invert_area { -a } else { a }
        };
        let userdata = pline.userdata.clone();

        Self {
            vertex_count: pline.vertex_count(),
            area,
            path_length: pline.path_length(),
            extents: pline.extents().unwrap(),
            userdata,
            is_closed: pline.is_closed(),
            orientation: pline.orientation(),
            has_repeat_position: pline.remove_repeat_pos(pos_equal_eps).is_some(),
        }
    }

    pub fn fuzzy_eq_eps(&self, other: &Self, eps: f64) -> bool {
        if self.vertex_count != other.vertex_count {
            return false;
        }
        if !self.area.fuzzy_eq_eps(other.area, eps) {
            return false;
        }
        if !self.path_length.fuzzy_eq_eps(other.path_length, eps) {
            return false;
        }
        if !aabb_fuzzy_eq_eps(&self.extents, &other.extents, eps) {
            return false;
        }
        if !userdata_sets_match(&self.userdata, &other.userdata) {
            return false;
        }
        true
    }

    pub fn fuzzy_eq_eps_abs_a(&self, other: &Self, eps: f64) -> bool {
        if self.vertex_count != other.vertex_count {
            return false;
        }
        if !self.area.abs().fuzzy_eq_eps(other.area.abs(), eps) {
            return false;
        }
        if !self.path_length.fuzzy_eq_eps(other.path_length, eps) {
            return false;
        }
        if !aabb_fuzzy_eq_eps(&self.extents, &other.extents, eps) {
            return false;
        }
        if !userdata_sets_match(&self.userdata, &other.userdata) {
            return false;
        }
        true
    }

    pub fn fuzzy_eq_eps_with_options(
        &self,
        other: &Self,
        eps: f64,
        options: &PropertyExpectationOptions,
    ) -> bool {
        if self.vertex_count != other.vertex_count {
            return false;
        }

        let area_matches = if options.compare_abs_area {
            self.area.abs().fuzzy_eq_eps(other.area.abs(), eps)
        } else {
            self.area.fuzzy_eq_eps(other.area, eps)
        };
        if !area_matches {
            return false;
        }

        if !self.path_length.fuzzy_eq_eps(other.path_length, eps) {
            return false;
        }
        if !aabb_fuzzy_eq_eps(&self.extents, &other.extents, eps) {
            return false;
        }
        if options.compare_user_data && !userdata_sets_match(&self.userdata, &other.userdata) {
            return false;
        }
        if options.compare_open_closed && self.is_closed != other.is_closed {
            return false;
        }
        if options.compare_orientation && self.orientation != other.orientation {
            return false;
        }
        if options.compare_repeat_vertices && self.has_repeat_position != other.has_repeat_position
        {
            return false;
        }
        true
    }
}

pub fn create_property_set<'a, I>(polylines: I, invert_area: bool) -> Vec<PlineProperties>
where
    I: IntoIterator<Item = &'a Polyline>,
{
    polylines
        .into_iter()
        .map(|pl| PlineProperties::from_pline(pl, invert_area))
        .collect()
}

pub fn create_property_set_with_eps<'a, I>(
    polylines: I,
    invert_area: bool,
    remove_redundant_eps: f64,
    pos_equal_eps: f64,
) -> Vec<PlineProperties>
where
    I: IntoIterator<Item = &'a Polyline>,
{
    polylines
        .into_iter()
        .map(|pl| {
            PlineProperties::from_pline_with_eps(
                pl,
                invert_area,
                remove_redundant_eps,
                pos_equal_eps,
            )
        })
        .collect()
}

pub fn userdata_sets_match(actual: &[u64], expected: &[u64]) -> bool {
    let mut sets_match = true;
    for datum in expected.iter() {
        if (!actual.contains(datum)) {
            sets_match = false;
            break;
        }
    }
    sets_match
}

pub fn property_sets_match(
    result_set: &[PlineProperties],
    expected_set: &[PlineProperties],
) -> bool {
    let mut sets_match = true;
    if result_set.len() != expected_set.len() {
        sets_match = false;
    } else {
        // using simple N^2 comparisons to compare property sets (sets are always relatively small,
        // e.g. N < 10)
        for properties_expected in expected_set {
            let match_count = result_set
                .iter()
                .filter(|properties_result| {
                    properties_expected
                        .fuzzy_eq_eps(properties_result, PlineProperties::PROP_CMP_EPS)
                })
                .count();

            if match_count != 1 {
                sets_match = false;
                break;
            }
        }
    }

    if !sets_match {
        eprintln!("result:\n{result_set:?}");
        eprintln!("expected:\n{expected_set:?}");
    }

    sets_match
}

pub fn property_sets_match_abs_a(
    result_set: &[PlineProperties],
    expected_set: &[PlineProperties],
) -> bool {
    let mut sets_match = true;
    if result_set.len() != expected_set.len() {
        sets_match = false;
    } else {
        // using simple N^2 comparisons to compare property sets (sets are always relatively small,
        // e.g. N < 10)
        for properties_expected in expected_set {
            let match_count = result_set
                .iter()
                .filter(|properties_result| {
                    properties_expected
                        .fuzzy_eq_eps_abs_a(properties_result, PlineProperties::PROP_CMP_EPS)
                })
                .count();

            if match_count != 1 {
                sets_match = false;
                break;
            }
        }
    }

    if !sets_match {
        eprintln!("result:\n{result_set:?}");
        eprintln!("expected:\n{expected_set:?}");
    }

    sets_match
}

pub fn property_sets_match_with_options(
    result_set: &[PlineProperties],
    expected_set: &[PlineProperties],
    eps: f64,
    options: &PropertyExpectationOptions,
) -> bool {
    let mut sets_match = true;
    if result_set.len() != expected_set.len() {
        sets_match = false;
    } else {
        // using simple N^2 comparisons to compare property sets (sets are always relatively small,
        // e.g. N < 10)
        for properties_expected in expected_set {
            let match_count = result_set
                .iter()
                .filter(|properties_result| {
                    properties_expected.fuzzy_eq_eps_with_options(properties_result, eps, options)
                })
                .count();

            if match_count != 1 {
                sets_match = false;
                break;
            }
        }
    }

    if !sets_match {
        eprintln!("result:\n{result_set:?}");
        eprintln!("expected:\n{expected_set:?}");
    }

    sets_match
}
