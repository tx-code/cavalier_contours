use crate::{
    core::{
        math::{
            Vector2, angle, bulge_from_angle, delta_angle_signed, line_line_intr,
            point_from_parametric,
        },
        traits::Real,
    },
    polyline::{
        PlineCreation, PlineOffsetProfileMode, PlineProfileOffsetError, PlineProfileOffsetOptions,
        PlineSource, PlineSourceMut,
    },
};

struct RawProfileSeg<T>
where
    T: Real,
{
    start: Vector2<T>,
    end: Vector2<T>,
    join_center: Vector2<T>,
    join_distance: T,
}

#[inline]
fn connection_bulge<T>(
    center: Vector2<T>,
    start_point: Vector2<T>,
    end_point: Vector2<T>,
    is_ccw: bool,
    pos_equal_eps: T,
) -> T
where
    T: Real,
{
    if start_point.fuzzy_eq_eps(end_point, pos_equal_eps) {
        return T::zero();
    }

    let a1 = angle(center, start_point);
    let a2 = angle(center, end_point);
    // Keep parity with existing rounded join orientation behavior:
    // `is_ccw=true` should sweep ccw from start to end.
    bulge_from_angle(delta_angle_signed(a1, a2, !is_ccw))
}

#[inline]
fn segment_offsets<T>(start_dist: T, end_dist: T, mode: PlineOffsetProfileMode) -> (T, T)
where
    T: Real,
{
    match mode {
        PlineOffsetProfileMode::LinearPerSegment => (start_dist, end_dist),
        PlineOffsetProfileMode::StepPerSegment => (start_dist, start_dist),
    }
}

#[inline]
fn join_two_offset_segs<T, O>(
    s1: &RawProfileSeg<T>,
    s2: &RawProfileSeg<T>,
    pos_equal_eps: T,
    result: &mut O,
) where
    T: Real,
    O: PlineSourceMut<Num = T>,
{
    match line_line_intr(s1.start, s1.end, s2.start, s2.end, pos_equal_eps) {
        crate::core::math::LineLineIntr::TrueIntersect { seg1_t, .. } => {
            let ip = point_from_parametric(s1.start, s1.end, seg1_t);
            result.add_or_replace(ip.x, ip.y, T::zero(), pos_equal_eps);
        }
        _ => {
            let sp = s1.end;
            let ep = s2.start;
            let bulge = connection_bulge(
                s1.join_center,
                sp,
                ep,
                s1.join_distance < T::zero(),
                pos_equal_eps,
            );
            result.add_or_replace(sp.x, sp.y, bulge, pos_equal_eps);
            result.add_or_replace(ep.x, ep.y, T::zero(), pos_equal_eps);
        }
    }
}

pub fn parallel_offset_profile_line_only<P, T, O>(
    polyline: &P,
    profile: &[T],
    options: &PlineProfileOffsetOptions<T>,
) -> Result<Vec<O>, PlineProfileOffsetError>
where
    P: PlineSource<Num = T> + ?Sized,
    T: Real,
    O: PlineCreation<Num = T>,
{
    let expected_profile_count = polyline.vertex_count();
    if profile.len() != expected_profile_count {
        return Err(PlineProfileOffsetError::InvalidProfileLength {
            expected: expected_profile_count,
            actual: profile.len(),
        });
    }

    if polyline.vertex_count() < 2 {
        return Ok(Vec::new());
    }

    if polyline.is_closed() {
        return Err(PlineProfileOffsetError::ClosedPolylineUnsupported);
    }

    let mut has_pos = false;
    let mut has_neg = false;
    for &d in profile {
        if d > options.pos_equal_eps {
            has_pos = true;
        } else if d < -options.pos_equal_eps {
            has_neg = true;
        }
    }
    if has_pos && has_neg {
        return Err(PlineProfileOffsetError::MixedOffsetSigns);
    }

    let mut raw_segs = Vec::with_capacity(polyline.segment_count());
    for (seg_start_index, (v1, v2)) in polyline.iter_segments().enumerate() {
        if !v1.bulge_is_zero() {
            return Err(PlineProfileOffsetError::ArcSegmentUnsupported { seg_start_index });
        }

        let dir = v2.pos() - v1.pos();
        if dir.length_squared() <= options.pos_equal_eps * options.pos_equal_eps {
            return Err(PlineProfileOffsetError::DegenerateSegment { seg_start_index });
        }
        let n = dir.safe_unit_perp();

        let seg_end_index = polyline.next_wrapping_index(seg_start_index);
        let (start_dist, end_dist) = segment_offsets(
            profile[seg_start_index],
            profile[seg_end_index],
            options.profile_mode,
        );

        let start = v1.pos() + n.scale(start_dist);
        let end = v2.pos() + n.scale(end_dist);
        raw_segs.push(RawProfileSeg {
            start,
            end,
            join_center: v2.pos(),
            join_distance: profile[seg_end_index],
        });
    }

    if raw_segs.is_empty() {
        return Ok(Vec::new());
    }

    let mut result = O::with_capacity(raw_segs.len() * 2 + 1, false);
    result.add(raw_segs[0].start.x, raw_segs[0].start.y, T::zero());

    if raw_segs.len() == 1 {
        result.add_or_replace(
            raw_segs[0].end.x,
            raw_segs[0].end.y,
            T::zero(),
            options.pos_equal_eps,
        );
    } else {
        for i in 0..raw_segs.len() - 1 {
            join_two_offset_segs(
                &raw_segs[i],
                &raw_segs[i + 1],
                options.pos_equal_eps,
                &mut result,
            );
        }

        let last = raw_segs.last().unwrap();
        result.add_or_replace(last.end.x, last.end.y, T::zero(), options.pos_equal_eps);
    }

    result.set_is_closed(false);

    result.set_userdata_values(polyline.get_userdata_values());
    if result.vertex_count() < 2 {
        return Ok(Vec::new());
    }

    Ok(vec![result])
}
