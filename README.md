# SQLite.mjs
Like sql.js except:
* SQLite is compiled to wasi instead of emscripten.  However, all IO functionality is disabled so (with enough optimization) no wasi imports end up in the finished .wasm module.
* Rust + wasm-bindgen: The wrapping is mostly done using Rust and wasm-bindgen instead of hand-written javascript.
* Asyncify: Runs the asyncify pass of the binaryen compiler.  This enables stack unwinding / rewinding.  The Rust code uses this to support async vfs implementations without the need for worker threads.  This means that sqlite needs to be compiled with multi-threading support (from sqlite's persepective, the asyncify coroutines look like threads).

## Dependencies
* wasm-pack
* wasm-opt
* wasi-sdk-19.0 (Download from https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-19 and extract here)
* Rust `wasm32-unknown-unknown` target (Should be installed when you first run build if you don't have it already)

## Build
`wasm-pack build --target web --no-typescript`
