# rust-wasm-ai-example

## install rust tools chain + plugin

### instal wasmEdge
```
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
```

### enable binary run on this session
```
source $HOME/.wasmedge/env
```

### install wasm plugin nn
```
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- --plugins wasi_nn-ggml
```

### install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### set rust complie target
```
rustup target add wasm32-wasi
```

## Simpple rust

### init project
```
cargo new rust-101
```

### run
```
cargo run
```