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
| `spatial_index` | 36 | `0.788` | Rust is faster in most pairs; native create small-shape cases are slightly slower. |
| `offset` | 18 | `0.876` | Rust is faster in all compared pairs. |
| `boolean` (`combine`) | 36 | `0.606` | Rust is faster in all compared pairs. |

Interpretation: ratio `< 1` means Rust faster, `> 1` means Rust slower.

## Spatial Index Outliers

Largest Rust-slower cases from this snapshot:

| Rust ID | Rust/C++ ratio | Delta |
|--------|-----------------|-------|
| `spatial_index/create/native/circle` | `1.141` | `+14.1%` |
| `spatial_index/create/native/diamond` | `1.093` | `+9.3%` |
| `spatial_index/create/native/square` | `1.076` | `+7.6%` |

No large Rust-slower outlier was observed outside these small native-create
cases.

## Notes

- This snapshot is directional evidence, not a CI threshold.
- Criterion and Google Benchmark use different harness implementations; absolute
  numbers are not interchangeable without that context.
- Raw local artifacts are under `target/bench-compare/` and are intentionally
  untracked.
