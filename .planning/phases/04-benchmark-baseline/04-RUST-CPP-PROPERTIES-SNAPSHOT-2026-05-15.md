# Rust/C++ Properties Benchmark Snapshot (2026-05-15)

## Scope

Local Rust/C++ comparison for property workloads:

- `properties/area/*`
- `properties/extents/*`
- `properties/path_length/*`
- `properties/winding_number_grid/*`

Modes include `native` and `no_arcs` where both sides expose matching profile
families.

## Commands

Rust:

```powershell
cargo bench -p cavalier_contours --bench geometry_baseline -- "properties/" --noplot --output-format bencher
```

C++:

```powershell
E:/Coding/CavalierContours/build-bench-local/areabenchmarks.exe --benchmark_filter='BM_area' --benchmark_format=json --benchmark_out=E:/Coding/cavalier_contours/target/bench-compare/cpp_area.json --benchmark_out_format=json --benchmark_time_unit=ns --benchmark_min_time=0.03s
E:/Coding/CavalierContours/build-bench-local/extentsbenchmarks.exe --benchmark_filter='BM_extents' --benchmark_format=json --benchmark_out=E:/Coding/cavalier_contours/target/bench-compare/cpp_extents.json --benchmark_out_format=json --benchmark_time_unit=ns --benchmark_min_time=0.03s
E:/Coding/CavalierContours/build-bench-local/pathlengthbenchmarks.exe --benchmark_filter='BM_pathLength' --benchmark_format=json --benchmark_out=E:/Coding/cavalier_contours/target/bench-compare/cpp_pathlength.json --benchmark_out_format=json --benchmark_time_unit=ns --benchmark_min_time=0.03s
E:/Coding/CavalierContours/build-bench-local/windingnumberbenchmarks.exe --benchmark_filter='BM_windingNumber100PtGrid' --benchmark_format=json --benchmark_out=E:/Coding/cavalier_contours/target/bench-compare/cpp_winding.json --benchmark_out_format=json --benchmark_time_unit=ns --benchmark_min_time=0.03s
```

## Summary

Using Rust bencher `ns/iter` vs C++ `real_time` converted to `ns/iter`:

| Metric | Pairs | Median Rust/C++ ratio | Rust slower count |
|--------|-------|------------------------|-------------------|
| `area` | 18 | `0.758` | `2/18` |
| `extents` | 18 | `0.737` | `1/18` |
| `path_length` | 18 | `0.695` | `0/18` |
| `winding_number_grid` | 18 | `0.704` | `1/18` |
| **All properties** | **72** | **`0.715`** | **`4/72`** |

Interpretation: ratio `< 1` means Rust faster, `> 1` means Rust slower.

## Highest Rust-Slower Cases (Initial Sweep)

| Key | Rust/C++ ratio | Delta |
|-----|-----------------|-------|
| `area|native|pathological_profile1_10` | `1.083` | `+8.3%` |
| `winding_number_grid|no_arcs|pathological_profile1_100` | `1.028` | `+2.8%` |
| `extents|native|diamond` | `1.013` | `+1.3%` |
| `area|native|pathological_profile1_50` | `1.001` | `+0.1%` |

## Targeted Long-Run Recheck

Targeted reruns with longer windows:

- Rust: `--measurement-time 2 --sample-size 50`
- C++: `--benchmark_min_time=1s`

| Case | Rust `ns/iter` | C++ `ns/iter` | Ratio |
|------|----------------|---------------|-------|
| `area|native|pathological_profile1_10` | `69` | `64.18` | `1.075` |
| `winding_number_grid|no_arcs|pathological_profile1_100` | `153427` | `164634` | `0.932` |
| `extents|native|diamond` | `12` | `11.60` | `1.034` |

The only consistent residual slower point is the small absolute gap in
`area|native|pathological_profile1_10`; other residuals are near parity or
flip direction under longer runs.

## Notes

- This is comparative evidence, not a CI threshold.
- Criterion and Google Benchmark use different harness implementations.
- Rust property benches in this snapshot remove unnecessary input `black_box`
  wrappers so measurements better reflect property-call cost.
- Raw local artifacts remain under `target/bench-compare/` and are untracked.
