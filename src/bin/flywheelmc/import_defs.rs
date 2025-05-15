use flywheelmc_common::prelude::*;
use flywheelmc_players::ServerMotd;
use flywheelmc_wasm::ImportFuncs;
use flywheelmc_wasm::{ WasmAnyPtr, WasmResult };
use protocol::value::Text;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    import_funcs.define("flywheel_system_set_motd", flywheel_system_set_motd);
}


async fn flywheel_system_set_motd(_in_motd : WasmAnyPtr, _motd_len : u32) -> WasmResult<()> {
    let motd : &str = todo!();
    trace!("Set server MOTD to {motd}");
    let text = Text::from_xml(motd, false, true);
    AsyncWorld.resource_scope::<ServerMotd, _>(|mut r_motd| { r_motd.0 = text; });
    Ok(())
}
