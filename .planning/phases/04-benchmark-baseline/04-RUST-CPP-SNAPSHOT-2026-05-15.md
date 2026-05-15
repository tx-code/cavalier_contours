# Rust/C++ Benchmark Snapshot (2026-05-15)

## Scope

Local snapshot comparing current Rust benchmark IDs in
`cavalier_contours/benches/geometry_baseline.rs` against old C++ benchmark
executables in `E:/Coding/CavalierContours/build-bench-local`.

Compared families:

- `spatial_index/*` (create + query_reuse_stack; native + no_arcs)
- `offset/*` (native + no_arcs)
- `boolean/shifted/*` and `boolean/coincident/*` (native + no_arcs)

## Commands

Rust (Criterion, bencher-format output):

```powershell
cargo bench -p cavalier_contours --bench geometry_baseline -- "spatial_index/" --noplot --output-format bencher
cargo bench -p cavalier_contours --bench geometry_baseline -- "offset/" --noplot --output-format bencher
cargo bench -p cavalier_contours --bench geometry_baseline -- "boolean/" --noplot --output-format bencher
```

C++ (Google Benchmark JSON output):

```powershell
E:/Coding/CavalierContours/build-bench-local/spatialindexbenchmarks.exe --benchmark_filter='BM_(createIndex|queryIndexReuseStack)' --benchmark_format=json --benchmark_out=E:/Coding/cavalier_contours/target/bench-compare/cpp_spatial.json --benchmark_out_format=json --benchmark_time_unit=ns --benchmark_min_time=0.03s
E:/Coding/CavalierContours/build-bench-local/offsetbenchmarks.exe --benchmark_filter='BM_offset' --benchmark_format=json --benchmark_out=E:/Coding/cavalier_contours/target/bench-compare/cpp_offset.json --benchmark_out_format=json --benchmark_time_unit=ns --benchmark_min_time=0.03s
E:/Coding/CavalierContours/build-bench-local/combinebenchmarks.exe --benchmark_filter='BM_combine(16Shifted|Coincident)' --benchmark_format=json --benchmark_out=E:/Coding/cavalier_contours/target/bench-compare/cpp_boolean.json --benchmark_out_format=json --benchmark_time_unit=ns --benchmark_min_time=0.03s
```

## Result Summary

Using Rust bencher `ns/iter` vs C++ `real_time` converted to `ns/iter`:

| Family | Compared pairs | Median Rust/C++ ratio | Main observation |
|--------|----------------|-----------------------|------------------|
| `spatial_index` | 36 | `0.767` | Rust is faster in most pairs; only two tiny native-create cases remain slightly slower (near parity). |
| `offset` | 18 | `0.856` | Rust is faster in all compared pairs. |
| `boolean` (`combine`) | 36 | `0.604` | Rust is faster in all compared pairs. |

Interpretation: ratio `< 1` means Rust faster, `> 1` means Rust slower.

## Spatial Index Outliers

Remaining Rust-slower cases from this snapshot:

| Rust ID | Rust/C++ ratio | Delta |
|--------|-----------------|-------|
| `spatial_index/create/native/circle` | `1.013` | `+1.3%` |
| `spatial_index/create/native/diamond` | `1.010` | `+1.0%` |

No additional Rust-slower outlier was observed.

### Targeted Long-Run Recheck

To verify the two tiny residual cases, we reran only
`spatial_index/create/native/{circle,diamond}` with longer windows:

- Rust: `--measurement-time 5 --sample-size 100`
- C++: `--benchmark_min_time=3s`

Observed values:

| Case | Rust `ns/iter` | C++ `ns/iter` | Ratio |
|------|----------------|---------------|-------|
| `circle` | `101` | `101.45` | `0.996` |
| `diamond` | `107` | `111.06` | `0.963` |

This indicates practical parity for the two residual tiny-shape create cases.

A same-day rerun with the same long-window settings also stayed at parity:

| Case | Rust `ns/iter` | C++ `ns/iter` | Ratio |
|------|----------------|---------------|-------|
| `circle` | `103` | `103.62` | `0.994` |
| `diamond` | `114` | `114.44` | `0.996` |

## Notes

- This snapshot is directional evidence, not a CI threshold.
- Criterion and Google Benchmark use different harness implementations; absolute
  numbers are not interchangeable without that context.
- `cavalier_contours` default features in this snapshot include
  `unsafe_optimizations` for `static_aabb2d_index`.
- Rust `spatial_index/create/*` benchmark IDs in this snapshot removed an
  unnecessary input `black_box` wrapper so very-small-case measurements better
  reflect index-build cost.
- Raw local artifacts are under `target/bench-compare/` and are intentionally
  untracked.
