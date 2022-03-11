# TCP server for WASI

This is a basic TCP server that prints the data that it receives. The idea can be extended to chat servers, etc.

At the time of writing this, WASI does not have support for multithreading. This server operates on a single thread and allows multiple connections through polling.

**It also depends on a preopened TCP socket.**

## Building

```
cargo build --release --target wasm32-wasi
```

## Running

```
wasmtime run --tcplisten 127.0.0.1:9000 target/wasm32-wasi/release/testwasi.wasm
```

## Preview

https://user-images.githubusercontent.com/45601318/157898958-ae2410fb-d4fb-4bd9-ba4d-87d57ffe547a.mp4

