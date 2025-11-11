# Rust Pool Sim - Research

## Overview

This document captures the technical research and rationale for the initial bootstrap of the `rust_pool_sim` project. The goal is to provide a lean, well-justified foundation for:

- A Rust-based core simulation targeting WebAssembly.
- A lightweight HTML5 Canvas frontend.
- A CI and validation pipeline focused on safety, performance, and binary size.

The choices below are intentionally conservative and stable to support long-term maintenance.

---

## 2D Physics Engine Options

Future iterations of the project may integrate a dedicated 2D physics engine instead of hand-rolled dynamics. The primary candidates considered are:

- Rapier2D
- nphysics2d
- bevy_xpbd_2d

### Comparison

| Engine         | Maintainer / Ecosystem         | Dimensionality | Maturity / Activity        | Bevy Integration | WASM Friendliness | Notes                                     |
|----------------|--------------------------------|----------------|----------------------------|------------------|-------------------|-------------------------------------------|
| Rapier2D       | Dimforge (successor to nphysics) | 2D/3D         | Actively maintained, modern | Optional crates  | Good              | High-performance, modern API, well-tested |
| nphysics2d     | Dimforge (legacy)              | 2D             | Deprecated/legacy          | No               | Limited           | Replaced by Rapier; not recommended       |
| bevy_xpbd_2d   | Community-driven (Bevy plugin)  | 2D             | Active in Bevy ecosystem   | Tight (Bevy-only) | Good w/ Bevy     | Strong if project standardizes on Bevy    |

### Conclusion

Rapier2D is preferred for future integration because:

- It is actively maintained and production-oriented.
- It offers strong WASM support and good performance.
- It supersedes nphysics and is not tightly coupled to a specific game engine.

nphysics2d is considered legacy and not recommended for new projects.
bevy_xpbd_2d is compelling in Bevy-centric environments but introduces a heavier stack than required for this simulation core.

References:

- https://rapier.rs
- https://github.com/dimforge/rapier
- https://github.com/dimforge/nphysics
- https://github.com/Jondolf/bevy_xpbd

---

## WASM Tooling: wasm-pack vs Trunk

Two primary approaches were considered for building and integrating Rust-generated WASM:

- `wasm-pack`
- `trunk`

### Comparison

| Tool       | Focus                               | Integration Model               | Config Complexity | Ecosystem Maturity | Notes                                               |
|------------|-------------------------------------|---------------------------------|-------------------|--------------------|-----------------------------------------------------|
| wasm-pack  | Library-style WASM for JS frontends | Outputs npm-compatible pkg/     | Low               | High               | Ideal for direct JS/Canvas usage, framework-agnostic |
| Trunk      | Full web bundler for Rust frontends | Manages HTML, assets, pipelines | Moderate          | High               | Great for Yew/Leptos-style apps; more moving pieces |

### Decision

`wasm-pack` is selected for the bootstrap because:

- It is simple and stable for library-style WASM modules.
- It aligns with a vanilla JS + Canvas frontend.
- It avoids adding an opinionated bundler at this stage.

Trunk remains a viable option if the project adopts a Rust-driven UI framework in the future.

References:

- https://rustwasm.github.io/wasm-pack/
- https://trunkrs.dev/

---

## Frontend Rendering: Canvas API vs PixiJS

For rendering, the initial prototype focuses on simplicity and minimal dependencies.

### Comparison

| Option        | Abstraction Level | Performance | Bundle Size Impact | Complexity | Notes                                               |
|---------------|-------------------|------------:|--------------------|-----------|-----------------------------------------------------|
| Canvas API    | Low (direct draw) | High        | Minimal            | Low       | Native browser API; ideal for small focused demos   |
| PixiJS        | Higher            | High        | Larger             | Medium    | Advanced batching, filters, and scene management    |

### Decision

The HTML5 Canvas 2D API is chosen because:

- It introduces zero external JS dependencies.
- It is sufficient for a small number of balls and simple effects.
- It aligns with the goal of keeping WASM and JS payloads small and auditable.

PixiJS may be reconsidered if advanced visual features or a complex scene graph become necessary.

References:

- https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API
- https://pixijs.com/

---

## Binary Size and Performance

Size and performance constraints are first-order design concerns:

- Release profile uses:
  - `opt-level = "z"`
  - `lto = true`
  - `codegen-units = 1`
  - `panic = "abort"`
- CI and `scripts/validate_setup.sh` enforce:
  - `wasm-pack build --target web --release --out-dir pkg`
  - Hard cap of 1 MiB (`1048576` bytes) for generated `.wasm` artifacts.
- Coding guidelines:
  - Prefer simple math and data layouts.
  - Avoid unnecessary heap allocations in tight loops.
  - Keep exported APIs minimal and composable.

These constraints ensure fast downloads, predictable performance, and good compatibility with constrained environments.

---

## Summary of Selected Stack

- Core language: Rust (stable, 2021 edition).
- Target: `cdylib` for WebAssembly + `rlib` for reuse.
- WASM binding: `wasm-bindgen` + `wasm-pack`.
- Frontend:
  - HTML5 + CSS
  - Canvas 2D API
  - Minimal ES module bootstrap (`frontend/main.js`).
- CI:
  - GitHub Actions:
    - Formatting, Clippy (`-D warnings`), tests.
    - WASM build and size gate.
    - `cargo audit` for vulnerability scanning.
- Future-ready:
  - Architecture compatible with integrating Rapier2D.
  - Deployment and hosting left flexible (any static file host).

This research underpins the lean bootstrap implemented in this repository and should be kept up to date as dependencies, tooling, or performance goals evolve.