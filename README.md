# Rust Pool Simulation WASM Bootstrap

This project (`rust_pool_sim`) is the bootstrap for a Rust + WASM-based pool simulation with a minimal, policy-enforced toolchain and CI.

The implementation is intentionally small and focused. Physics and advanced features are out of scope for the bootstrap and will be layered on later.

## Components

- Rust library crate (compiled to native + `wasm32-unknown-unknown`):
  - Core simulation state and step functions.
  - `wasm-bindgen` exports for use from the browser.
- WASM build:
  - Built via `wasm-pack` with `--target web --release --out-dir pkg`.
  - Must produce a `.wasm` artifact under 1 MiB.
- Frontend:
  - Static files in `frontend/`
    - `index.html`
    - `style.css`
    - `main.js`
  - Integrates the generated WASM:
    - Loads `pkg/` output.
    - Uses Canvas for visualization hooks.
- Tooling and policy:
  - `scripts/validate_setup.sh` orchestrates all local checks.
  - GitHub Actions CI (`.github/workflows/ci.yml`) mirrors the same checks.
  - Security and quality gates are strict and must not be weakened.

## Validation Pipeline

The required validation steps (both locally and in CI) are:

1. `cargo generate-lockfile`
2. `cargo check`
3. `cargo fmt --all -- --check`
4. `cargo clippy --all-targets --all-features -- -D warnings`
5. `cargo test --all-targets`
6. `wasm-pack build --target web --release --out-dir pkg`
7. WASM size check:
   - The built `.wasm` in `pkg/` must be:
     - Found successfully.
     - Strictly less than `1,048,576` bytes (1 MiB).
8. `cargo audit`:
   - Required for security scanning of dependencies.

These steps are wired together in [`scripts/validate_setup.sh`](rust_pool_sim/scripts/validate_setup.sh:1) and in CI.

## Environment Constraints

In this environment snapshot:

- `git` is available.
- `cargo`, `wasm-pack`, and `cargo-audit` are currently NOT available on `PATH`.
- As a result:
  - Actual compilation, linting, testing, WASM build, size verification, and audit cannot be executed here.
  - The design **keeps all of these as required checks**; there is no relaxation of policy.

The `scripts/validate_setup.sh` script:

- Detects missing required tools.
- Fails early with a clear message if `cargo`, `wasm-pack`, or `cargo-audit` are not installed.
- When all tools are present, it:
  - Runs the full validation pipeline.
  - Enforces the WASM size budget.
  - Requires `cargo-audit` to pass.

## Local Setup Instructions

To run the full validation on your own machine:

1. Install Rust (stable) and `cargo`:

   - Follow instructions at: https://rustup.rs/

2. Add the `wasm32-unknown-unknown` target:

   - `rustup target add wasm32-unknown-unknown`

3. Install `wasm-pack`:

   - See: https://rustwasm.github.io/wasm-pack/installer/
   - Example (if using cargo-binstall or similar):
     - `cargo install wasm-pack` (or use official installer)

4. Install `cargo-audit`:

   - `cargo install cargo-audit`

5. From the project root (`rust_pool_sim/`), run:

   - `bash scripts/validate_setup.sh`

If all tools are installed correctly and the code remains consistent with this bootstrap, you should see:

- All checks completing successfully.
- A final message:
  - `All checks PASSED`

## Git and CI

- The repository is intended to be initialized with:

  - `.gitignore`
  - `Cargo.toml`
  - `Cargo.lock` (generated via `cargo generate-lockfile`)
  - `src/`
  - `frontend/`
  - `.github/`
  - `scripts/`
  - `README.md`
  - `RESEARCH.md`

- Initial commit (Conventional Commits):

  - Subject:
    - `feat: initial project structure and WASM bootstrap`
  - Body should summarize:
    - Rust library + WASM setup.
    - Frontend Canvas integration.
    - Research-backed tech choices (see `RESEARCH.md`).
    - CI and `scripts/validate_setup.sh` gates:
      - `fmt`, `clippy`, `test`, `wasm-pack`, size budget, `cargo audit`.

## Notes

- The current configuration and scripts are authoritative for the bootstrap.
- Any future changes must:
  - Preserve or strengthen validation, security, and size constraints.
  - Maintain compatibility with stable Rust and `wasm32-unknown-unknown`.
  - Keep the `wasm-pack` + web target integration intact.