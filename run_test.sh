reset
cargo build &&
./target/debug/flywheelmc -b=127.0.0.1:25580 -l=trace,flywheelmc_players::conn=debug test.wasm
