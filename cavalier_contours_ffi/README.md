## Summary

C foreign function interface (FFI) for the [Cavalier Contours](https://github.com/jbuckmccready/cavalier_contours) library.


## Usage

> [!NOTE]
> Run all these commands from the root of the cavalier contours repository/workspace.

You can generate the root C header file using
[cbindgen](https://github.com/mozilla/cbindgen) with the following command:

```sh
cbindgen --crate cavalier_contours_ffi --output cavalier_contours_ffi.h
```

The `.so` library file (`.dylib` on macOS, `.dll` on Windows) is built just by building the crate from cargo:

```sh
cargo build -p cavalier_contours_ffi --release
```

The resulting shared library file will be located in `/target/release`, the file name will be `libcavalier_contours_ffi.so` on Linux, `cavalier_contours_ffi.dll` on Windows, and something similar on macOS (ending in `.dylib`).

## Compatibility Notes

- The C FFI is intentionally versioned as a stable C ABI surface.
- New Rust-only convenience helpers (for example `rect_clip`) may be available
  in the Rust crate before a dedicated C wrapper is added.
- Regenerate `cavalier_contours_ffi.h` only when the C ABI surface changes.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
