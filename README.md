# UBRN Demo Project for Rust -> WASM bindings

## Setup

We usually use [bun](https://bun.com/) for package management.

Install dependencies:

```bash
  bun install
```

Install WASM specific targets:

```bash
rustup target add \
    wasm32-unknown-unknown  
```

Install wasm-bindgen-cli:

```bash
cargo install wasm-bindgen-cli
```

Download sample uniffi rust code defined in `ubrn.config.yaml`:

```bash
bun ubrn:checkout
```

You can find the uniffi rust code under `rust_modules/uniffi-starter/rust`.

## Create WASM Bindings for rustmodules

We have our rust library with uniffi bindings set up.
Now, we generate typescript bindings using ubrn.

Build the WASM bindings:

```bash
bun ubrn:web
```

This creates rust_modules/wasm containing the uniffi rust code translated to wasm-bindgen.
It then uses wasm-bindgen-cli to create the typescript bindings under the specified location in `ubrn.config.yaml`.
In this case the generated files are located at `src/generated/web`.

```yaml
rust:
  repo: https://github.com/jhugman/uniffi-starter.git
  branch: jhugman/bump-uniffi-to-0.29
  manifestPath: rust/foobar/Cargo.toml
web:
  ts: src/generated/web
```

Instead of defining a repo we can also define a directory containing the uniffi code.
