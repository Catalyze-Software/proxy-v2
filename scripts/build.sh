# Build script proxy canister

# Generate candid
cargo test candid -p proxy

# Build wasm
cargo build -p proxy --release --target wasm32-unknown-unknown

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/proxy.wasm > target/wasm32-unknown-unknown/release/proxy.wasm.gz

# Copy wasm
cp target/wasm32-unknown-unknown/release/proxy.wasm.gz wasm/proxy.wasm.gz
