# old Build steps

1. cargo build --target wasm32-unknown-unknown --release --example test
2. wasm2luau ./target/wasm32-unknown-unknown/release/examples/test.wasm > ./roblox/wasm.luau
3. rojo build --output build.rbxl

# New And Improved

1. Get real
2. Run this

```bash
# you must have rojo-script installed :woo:
./build.sh
```

# Serving the massive build.lua file

```bash
# don't use localtunnel because it leaks your ip in a header
ssh -R 80:localhost:8080 serveo.net
```
