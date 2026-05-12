# Stack Research

**Domain:** Arc-aware 2D computational geometry library absorption
**Researched:** 2026-05-12
**Confidence:** HIGH for local stack, MEDIUM for external comparison scope

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust | 1.88 MSRV, 2024 edition | Main geometry kernel | Existing crate uses this and enforces safe core code |
| Cargo workspace | resolver 2 | Multi-crate coordination | Current repository already separates core, FFI, UI, and examples |
| C ABI via Rust FFI crate | current repo version 0.7.0 | Integration with C/C++ and other languages | Keeps Rust as mainline while supporting migration users |
| C++ reference repos | C++14 old CavalierContours, C++17 Clipper2 | Reference algorithms, tests, and benchmarks | Useful as source material without making C++ a runtime dependency |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| static_aabb2d_index | 2.0 | Broad-phase spatial indexing | Offset, boolean, containment, and intersection acceleration |
| num-traits | 0.2 | Generic numeric traits | Existing generic geometry operations |
| serde | optional | Serialization | UI state, fixtures, and persisted regression cases |
| cbindgen | external tool | Header generation | Only when FFI ABI changes |
| Clipper2 C++ | local `E:\Coding\Clipper2` main | Polygon-only oracle | Difference, union, intersection, XOR, offsets, rect clipping, orientation |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| cargo test --workspace | Full Rust test gate | Run before submitting algorithm or FFI changes |
| cargo clippy --all-targets -- -D warnings | Lint gate | CI treats warnings as failures |
| cargo fmt --all --check | Formatting gate | Keep rustfmt defaults |
| cargo doc --workspace --no-deps | Public API docs | CI uses rustdoc warnings as errors |
| Google Benchmark / CMake in old C++ repo | Historical benchmark source | Mine profiles and cases; do not make required for normal Rust dev |

## Installation

```powershell
cargo test --workspace
cargo clippy --all-targets -- -D warnings
cargo doc --workspace --no-deps
```

FFI header regeneration remains explicit:

```bash
cbindgen --crate cavalier_contours_ffi -l c -o cavalier_contours_ffi.h
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Rust mainline | C++ mainline revival | Only if Rust ownership direction is reversed |
| Local Rust differential harness | Linking Clipper2 as production backend | Only for an experimental oracle harness, not library runtime |
| serde fixtures | Ad hoc text fixtures | Only for one-off debugging that will not become regression coverage |
| Existing AABB index | New spatial index abstraction | Only if profiling shows index structure is the bottleneck |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Clipper2 triangulation | Explicitly out of scope and README warns it is buggy | Defer triangulation entirely |
| Arc approximation as default behavior | Loses the crate's core value for CAD/CAM-like arcs | Preserve bulge-aware algorithms |
| C++ runtime dependency in the Rust crate | Blurs ownership, packaging, and safety boundaries | Keep C++ in tests/tools/oracle harnesses |
| Broad UI rewrite | User scoped UI to feature-driven demo needs | Add demo support only when new geometry needs visualization |

## Stack Patterns by Variant

**If comparing polygon-only cases:**
- Convert eligible Rust polylines to polygon paths with explicit approximation metadata.
- Compare area, winding/orientation, containment, vertex counts, and tolerance bands.

**If comparing arc-aware cases:**
- Use old C++ CavalierContours as historical behavior reference.
- Prefer property comparisons over exact vertex-by-vertex equality.

**If changing FFI:**
- Update Rust FFI tests, regenerate `cavalier_contours_ffi.h`, and document ABI impact.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| cavalier_contours 0.7.0 | Rust 1.88+ | Workspace MSRV is explicit |
| cavalier_contours_ffi | root generated header | Header must match exported structs/functions |
| Clipper2 C++ | C++17 | Oracle tooling should isolate this from normal Rust builds |
| old CavalierContours | C++14 | Historical reference and benchmark source |

## Sources

- `Cargo.toml` and crate manifests - current Rust stack
- `.planning/codebase/STACK.md` - current codebase map
- `E:\Coding\CavalierContours\README.md` - historical algorithm and benchmark context
- `E:\Coding\Clipper2\README.md` and `CPP/` tree - Clipper2 scope and local structure

---
*Stack research for: arc-aware geometry absorption*
*Researched: 2026-05-12*
