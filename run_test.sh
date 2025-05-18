reset
cargo build &&
(cd ../flywheel-sys-rust/examples/hello_world; ./build.sh) &&
./target/debug/flywheelmc -b=127.0.0.1:25580 -l=trace,flywheelmc_players::conn=debug ../flywheel-sys-rust/examples/hello_world/target/wasm32-unknown-unknown/release/hello_world.wasm
