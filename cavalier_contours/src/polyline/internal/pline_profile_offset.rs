use crate::{
    core::{
        math::{
            Vector2, angle, bulge_from_angle, delta_angle, delta_angle_signed, line_line_intr,
            point_from_parametric, point_on_circle,
        },
        traits::Real,
    },
    polyline::{
        PlineCreation, PlineOffsetProfileMode, PlineProfileOffsetError, PlineProfileOffsetOptions,
        PlineSource, PlineSourceMut, Polyline, seg_arc_radius_and_center,
    },
};
use num_traits::NumCast;

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
fn segment_profile_value<T>(start_dist: T, end_dist: T, mode: PlineOffsetProfileMode, t: T) -> T
where
    T: Real,
{
    match mode {
        PlineOffsetProfileMode::LinearPerSegment => start_dist + (end_dist - start_dist) * t,
        PlineOffsetProfileMode::StepPerSegment => start_dist,
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

fn validate_profile<T>(profile: &[T], pos_equal_eps: T) -> Result<(), PlineProfileOffsetError>
where
    T: Real,
{
    let mut has_pos = false;
    let mut has_neg = false;
    for &d in profile {
        if d > pos_equal_eps {
            has_pos = true;
        } else if d < -pos_equal_eps {
            has_neg = true;
        }
    }
    if has_pos && has_neg {
        return Err(PlineProfileOffsetError::MixedOffsetSigns);
    }

    Ok(())
}

fn linearize_arcs_with_profile<P, T>(
    polyline: &P,
    profile: &[T],
    options: &PlineProfileOffsetOptions<T>,
) -> Result<(Polyline<T>, Vec<T>), PlineProfileOffsetError>
where
    P: PlineSource<Num = T> + ?Sized,
    T: Real,
{
    if options.arc_approx_error <= T::zero() {
        return Err(PlineProfileOffsetError::InvalidArcApproxError);
    }

    let mut linearized =
        Polyline::<T>::with_capacity(polyline.vertex_count(), polyline.is_closed());
    let mut expanded_profile = Vec::with_capacity(polyline.vertex_count());
    let abs_error = options.arc_approx_error.abs();

    for (seg_start_index, (v1, v2)) in polyline.iter_segments().enumerate() {
        let seg_end_index = polyline.next_wrapping_index(seg_start_index);
        let start_dist = profile[seg_start_index];
        let end_dist = profile[seg_end_index];

        linearized.add(v1.x, v1.y, T::zero());
        expanded_profile.push(start_dist);

        if v1.bulge_is_zero() {
            continue;
        }

        let (arc_radius, arc_center) = seg_arc_radius_and_center(v1, v2);
        if arc_radius.fuzzy_lt(abs_error) {
            continue;
        }

        let start_angle = angle(arc_center, v1.pos());
        let end_angle = angle(arc_center, v2.pos());
        let angle_diff = delta_angle(start_angle, end_angle).abs();
        let seg_sub_angle = T::two() * (T::one() - abs_error / arc_radius).acos().abs();
        let seg_count = (angle_diff / seg_sub_angle).ceil();
        let seg_angle_offset = if v1.bulge_is_neg() {
            -angle_diff / seg_count
        } else {
            angle_diff / seg_count
        };

        let usize_count = seg_count
            .to_usize()
            .ok_or(PlineProfileOffsetError::ArcApproximationFailed { seg_start_index })?;

        for i in 1..usize_count {
            let angle_pos = <T as NumCast>::from(i)
                .ok_or(PlineProfileOffsetError::ArcApproximationFailed { seg_start_index })?;
            let t = angle_pos / seg_count;
            let angle = angle_pos * seg_angle_offset + start_angle;
            let pos = point_on_circle(arc_radius, arc_center, angle);
            linearized.add(pos.x, pos.y, T::zero());
            expanded_profile.push(segment_profile_value(
                start_dist,
                end_dist,
                options.profile_mode,
                t,
            ));
        }
    }

    if !polyline.is_closed() {
        let last = polyline.last().unwrap();
        linearized.add(last.x, last.y, T::zero());
        expanded_profile.push(*profile.last().unwrap());
    }

    Ok((linearized, expanded_profile))
}

fn parallel_offset_profile_line_only<P, T, O>(
    polyline: &P,
    profile: &[T],
    options: &PlineProfileOffsetOptions<T>,
) -> Result<Vec<O>, PlineProfileOffsetError>
where
    P: PlineSource<Num = T> + ?Sized,
    T: Real,
    O: PlineCreation<Num = T>,
{
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

    let is_closed = polyline.is_closed();
    let mut result = O::with_capacity(raw_segs.len() * 2 + 1, is_closed);
    result.add(raw_segs[0].start.x, raw_segs[0].start.y, T::zero());

    if raw_segs.len() == 1 {
        result.add_or_replace(
            raw_segs[0].end.x,
            raw_segs[0].end.y,
            T::zero(),
            options.pos_equal_eps,
        );
    } else if is_closed {
        for i in 0..raw_segs.len() {
            let next = (i + 1) % raw_segs.len();
            join_two_offset_segs(
                &raw_segs[i],
                &raw_segs[next],
                options.pos_equal_eps,
                &mut result,
            );
        }

        if result.vertex_count() > 1
            && result
                .at(0)
                .pos()
                .fuzzy_eq_eps(result.last().unwrap().pos(), options.pos_equal_eps)
        {
            result.remove_last();
        }

        if result.vertex_count() > 1
            && result
                .at(0)
                .pos()
                .fuzzy_eq_eps(result.at(1).pos(), options.pos_equal_eps)
        {
            result.remove(0);
        }
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

    result.set_is_closed(is_closed);
    if result.vertex_count() < 2 {
        return Ok(Vec::new());
    }

    Ok(vec![result])
}

pub fn parallel_offset_profile<P, T, O>(
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

    validate_profile(profile, options.pos_equal_eps)?;

    let has_arcs = polyline.iter_segments().any(|(v1, _)| !v1.bulge_is_zero());
    let mut result: Vec<O> = if has_arcs {
        let (linearized, expanded_profile) =
            linearize_arcs_with_profile(polyline, profile, options)?;
        parallel_offset_profile_line_only(&linearized, &expanded_profile, options)?
    } else {
        parallel_offset_profile_line_only(polyline, profile, options)?
    };

    for cursor in result.iter_mut() {
        cursor.set_userdata_values(polyline.get_userdata_values());
    }

    Ok(result)
}
