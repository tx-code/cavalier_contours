# Plan 04-02 Summary: Historical Benchmark Family Mapping

**Completed:** 2026-05-12
**Status:** Complete

## Changes

- Reworked `geometry_baseline` around historical old C++ profile families:
  `square`, `diamond`, `circle`, `rounded_rectangle`, `profile1`, `profile2`,
  and `pathological_profile1` with segment counts 10, 25, 50, and 100.
- Added separate native and no-arcs benchmark groups. No-arcs inputs use
  `arcs_to_approx_lines(0.01)` before timing, so conversion cost is excluded.
- Mapped old offset, combine/boolean, spatial index, area, extents, path length,
  and winding-number workload families into Rust benchmark IDs.
- Created `04-BENCHMARK-MAP.md` with old repo commit, license, source paths,
  operation mapping, and cost policy.

## Verification

- `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` passed.
- Source/map checks found all required old C++ benchmark source paths and
  profile family names.
- `git diff --check` passed.
- `git status --short -- target` reported no generated benchmark output to
  commit.

## Notes

- `intersections/current_only/*` remains current-Rust-only coverage for BEN-01
  because no historical old C++ intersection benchmark file was found.
- `cargo fmt --all` was run after the mapping patch.

