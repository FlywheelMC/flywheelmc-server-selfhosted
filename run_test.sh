EXAMPLE_NAME=big_clock

reset
cargo build &&
(cd ../flywheel-sys-rust/examples/${EXAMPLE_NAME}; ./build.sh) &&
./target/debug/flywheelmc -b=127.0.0.1:25580 -l=trace,flywheelmc_players::conn=debug,flywheelmc_players::world=debug,flywheelmc_wasm::runner::event=debug ../flywheel-sys-rust/examples/${EXAMPLE_NAME}/target/wasm32-unknown-unknown/release/${EXAMPLE_NAME}.wasm
