# Phase 04 Historical Benchmark Map

**Created:** 2026-05-12

## Source Snapshot

| Field | Value |
|-------|-------|
| Old repo | `E:/Coding/CavalierContours` |
| Commit | `31a012947aa2e7e9474e2ec90502825afe8b99a4` |
| License | `MIT` |
| Rust target | `cavalier_contours/benches/geometry_baseline.rs` |

## Profile Families

| Old C++ family | Rust profile ID | Native arc case | No-arcs case | Notes |
|----------------|-----------------|-----------------|--------------|-------|
| `square` | `square` | yes | not applicable | Line-only profile from `benchmarkprofiles.h`. |
| `diamond` | `diamond` | yes | not applicable | Line-only profile from `benchmarkprofiles.h`. |
| `circle` | `circle` | yes | yes | No-arcs uses `arcs_to_approx_lines(0.01)` before timing. |
| `roundedRectangle` | `rounded_rectangle` | yes | yes | Rust ID uses snake_case naming. |
| `profile1` | `profile1` | yes | yes | Mixed line/arc historical profile. |
| `profile2` | `profile2` | yes | yes | Mixed line/arc historical profile. |
| `pathologicalProfile1` | `pathological_profile1_10`, `_25`, `_50`, `_100` | yes | yes | Segment counts match old benchmark args. |

## Operation Mapping

| Old source | Old workload | Rust benchmark group | Cost notes |
|------------|--------------|----------------------|------------|
| `tests/benchmarks/offsetbenchmarks.cpp` | positive and negative offset loop through `offsetCount * offsetDelta` | `offset/native/*`, `offset/no_arcs/*` | Offset execution included; profile construction excluded. |
| `tests/benchmarks/combinebenchmarks.cpp` | 16 shifted copies with Union, Exclude, Intersect, XOR | `boolean/shifted/native/*`, `boolean/shifted/no_arcs/*` | Shifted copy setup excluded. |
| `tests/benchmarks/combinebenchmarks.cpp` | coincident combine with Union, Exclude, Intersect, XOR | `boolean/coincident/native/*`, `boolean/coincident/no_arcs/*` | Coincident input setup excluded. |
| `tests/benchmarks/spatialindexbenchmarks.cpp` | approximate spatial index creation | `spatial_index/create/native/*`, `spatial_index/create/no_arcs/*` | Index construction is the measured operation. |
| `tests/benchmarks/spatialindexbenchmarks.cpp` | query every segment AABB with reusable stack | `spatial_index/query_reuse_stack/native/*`, `spatial_index/query_reuse_stack/no_arcs/*` | Index and scratch setup excluded; query loop included. |
| `tests/benchmarks/areabenchmarks.cpp` | `getArea` | `properties/area/native/*`, `properties/area/no_arcs/*` | Property call included. |
| `tests/benchmarks/extentsbenchmarks.cpp` | `getExtents` | `properties/extents/native/*`, `properties/extents/no_arcs/*` | Property call included. |
| `tests/benchmarks/pathlengthbenchmarks.cpp` | `getPathLength` | `properties/path_length/native/*`, `properties/path_length/no_arcs/*` | Property call included. |
| `tests/benchmarks/windingnumberbenchmarks.cpp` | 10x10 expanded extents grid | `properties/winding_number_grid/native/*`, `properties/winding_number_grid/no_arcs/*` | Grid setup excluded; 100 winding calls included. |

## Current-Only Coverage

`intersections/current_only/*` covers current Rust intersection behavior for
BEN-01. The historical old C++ benchmark suite does not include a dedicated
intersection benchmark source, so these cases are intentionally not mapped to an
old source path.

## Cost Policy

- Native arc benchmark IDs measure current Rust arc-aware behavior.
- No-arcs benchmark IDs derive inputs with `arcs_to_approx_lines(0.01)` before
  the timed loop. Conversion cost is excluded.
- Query-reuse-stack IDs mirror old C++ `QuerySetup`: prebuilt index plus reused
  `query_results` and `query_stack` buffers are created outside the timed loop.
- Clipper2 runtime/oracle cost is excluded from Phase 4 and remains Phase 5
  policy.
- Generated Criterion output under `target/criterion` is local-only evidence and
  is not committed.
