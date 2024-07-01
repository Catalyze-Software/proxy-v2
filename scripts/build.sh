cargo test candid -p proxy
cargo build -p proxy --release --target wasm32-unknown-unknown
gzip -c target/wasm32-unknown-unknown/release/proxy.wasm > target/wasm32-unknown-unknown/release/proxy.wasm.gz

mkdir -p wasm

# Copy wasm
cp target/wasm32-unknown-unknown/release/proxy.wasm.gz wasm/proxy.wasm.gz
