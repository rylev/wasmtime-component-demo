# WebAssembly Components Demo

# Prerequisites

You will need the following tooling installed:
* [`rustup`](https://rustup.rs) which call gives you cargo/rustc
* The `wasm32-unknown-unknown` target which you can install with `rustup target add wasm32-unknown-unknown`
* `wasm-tools` which you can install with `cargo install wasm-tools`

## Running

First, build the wasm module:

```bash
cargo build --target wasm32-unknown-unknown --release -p markdown
```

The convert it into a component:

```bash
wasm-tools component new ./target/wasm32-unknown-unknown/release/markdown.wasm -o markdown-component.wasm
```

Then run the demo:

```bash
cargo run markdown-component.wasm
```

## Thanks

Thank you to everyone who is working on WebAssembly tooling.

Additionally, this is based on the following work:
* https://github.com/r-muhairi/component-model-demo
* https://blog.mediosz.club/2022/11/17/how-to-use-wit-bindgen
