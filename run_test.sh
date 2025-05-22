# Use this example from flywheel-sys-rs
EXAMPLE_NAME=big_clock

# Clear Terminal
clear

# Build flywheelmc-server-selfhosted
cargo build || exit 1

# Build the selected example script
(cd ../flywheel-sys-rust/examples/${EXAMPLE_NAME}; ./build.sh) || exit 1

# Start the server
./target/debug/flywheelmc \
  -b=127.0.0.1:25565 \
  -l="trace,\
       flywheelmc_players::conn=debug,\
       flywheelmc_players::world=debug,\
       flywheelmc_wasm::runner::event=debug" \
  ../flywheel-sys-rust/examples/${EXAMPLE_NAME}/target/wasm32-unknown-unknown/release/${EXAMPLE_NAME}.wasm

