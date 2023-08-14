# old Build steps

1. cargo build --target wasm32-unknown-unknown --release --example test
2. wasm2luau ./target/wasm32-unknown-unknown/release/examples/test.wasm > ./roblox/wasm.luau
3. rojo build --output build.rbxl

# New And Improved

1. Get real
2. Run this

```bash
# you must have rojo-script installed :woo:
RUSTFLAGS="--remap-path-prefix $HOME=~ --remap-path-prefix $PWD=." cargo build --target wasm32-unknown-unknown --release --example test && wasm2luau ./target/wasm32-unknown-unknown/release/examples/test.wasm > ./roblox/wasm.luau && rojo build -o build.rbxm && ../dev/rojo/target/release/rojo-script --file build.rbxm --output out.lua --runtime lua-sandbox
```
