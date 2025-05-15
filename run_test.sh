reset
cargo build &&
./target/debug/flywheelmc -b=127.0.0.1:25580 -l=trace,flywheelmc_players::conn=debug ../example-rust/target/wasm32-unknown-unknown/release/example_rust.wasm
