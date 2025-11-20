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

You can find the uniffi rust code under `rust_modules/uniffi-starter/rust`.

## Create WASM Bindings for rustmodules

We have our rust library with uniffi bindings set up.
Now, we generate typescript bindings using ubrn.

Build the WASM bindings:

```bash
bun ubrn:web
```
