# Cookie refuser

A simple web extension that refuse optional cookies.

She has been developed in Rust (webassembly).

## How to build

### Prerequisites

We assume that you have `cargo` and `rustup` installed. If not, you can install them by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

### Building the web extension

To build the web extension, you need to have `wasm-pack` installed.
```bash
cargo install wasm-pack
```

Then, you can build the web extension with the following command:
```bash
wasm-pack build --target web --out-dir extension/wasm
```

### Using the web extension

To use the web extension, you need to install it in your browser.
In chrome, go to `chrome://extensions/`, enable the developer mode, click on "Load unpacked" and select the `extension` directory.
Congrats, you can now use the web extension!
