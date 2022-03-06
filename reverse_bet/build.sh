RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/reverse_bet.wasm ./res/reverse_bet.wasm
