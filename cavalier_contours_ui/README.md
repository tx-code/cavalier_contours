## Summary

This is the ui app for visualizing, testing, and demonstrating the cavalier_contours crate.

Github CI publishes latest on `master` branch to GitHub Pages.

- This repo demo: [https://tx-code.github.io/cavalier_contours/](https://tx-code.github.io/cavalier_contours/)
- Fork demo default: `https://<github-username>.github.io/<repo-name>/`

If you want a custom domain for Pages, set repository variable `PAGES_PUBLIC_URL`
to that domain URL (for example `https://demo.example.com`).

The app is built with:
- [egui](https://github.com/emilk/egui) (immediate mode GUI library)
- [egui_plot](https://github.com/emilk/egui_plot) (plotting widget for egui)
- [lyon](https://github.com/nical/lyon) (path tessellation library for filling concave and complex polygons)


## Running Native Locally

```sh
cargo run
```

## Run Web Locally

> [!IMPORTANT]
> You need to have trunk installed to run the web ui locally (install docs [here](https://trunkrs.dev/guide/getting-started/installation.html)).
> Compiling from source:
> ```sh
> cargo install trunk --locked
> ```
> You will also need the rust wasm target installed, using rustup:
> ```sh
> rustup target add wasm32-unknown-unknown
> ```

```sh
trunk serve
```

Then go to [http://localhost:8080/#dev](http://localhost:8080/#dev) in your browser (`/#dev` is important to force not loading a stale cached version).

## WASM Output FAQ

If you build and inspect the generated JS bundle and only see `default` and `initSync`, that is expected for this UI app build.

- `cavalier_contours_ui` is an eframe/trunk web application entrypoint, not a JS-callable API surface.
- For running the demo, use `trunk serve` or `trunk build`; do not call exported geometry functions directly from the generated JS bundle.
- If you need direct JS-callable geometry functions, create a separate `wasm-bindgen` wrapper crate that exports the functions you need from `cavalier_contours`.
