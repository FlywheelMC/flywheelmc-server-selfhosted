# Use this example from flywheel-sys
EXAMPLE_LANG=zig
EXAMPLE_NAME=hello_world

# Clear Terminal
clear

# Build flywheelmc-server-selfhosted
cargo build || exit 1

# Build the selected example script
(cd ../flywheel-sys-${EXAMPLE_LANG}/examples/${EXAMPLE_NAME}; ./build.sh) || exit 1

# Figure out where the outputted file is
case $EXAMPLE_LANG in
    "rust")
        BUILD_PATH=target/wasm32-unknown-unknown/release/
        ;;
    "zig")
        BUILD_PATH=zig-out/bin/
        ;;
esac

# Start the server
./target/debug/flywheelmc \
  -b=127.0.0.1:25565 \
  -l="trace,\
       flywheelmc_players::conn=debug,\
       flywheelmc_players::world=debug,\
       flywheelmc_wasm::runner::event=debug" \
  ../flywheel-sys-${EXAMPLE_LANG}/examples/${EXAMPLE_NAME}/${BUILD_PATH}${EXAMPLE_NAME}.wasm
