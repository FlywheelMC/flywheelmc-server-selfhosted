use flywheelmc_common::prelude::*;
use flywheelmc_players::ServerMotd;
use flywheelmc_wasm::ImportFuncs;
use flywheelmc_wasm::{ WasmAnyPtr, WasmResult };
use protocol::value::Text;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    import_funcs.define("flywheel_system_set_motd", flywheel_system_set_motd );
    import_funcs.define("flywheel_trace",    flywheel_trace    );
    import_funcs.define("flywheel_debug",    flywheel_debug    );
    import_funcs.define("flywheel_info ",    flywheel_info     );
    import_funcs.define("flywheel_pass ",    flywheel_pass     );
    import_funcs.define("flywheel_warn ",    flywheel_warn     );
    import_funcs.define("flywheel_error",    flywheel_error    );
    import_funcs.define("flywheel_fatal",    flywheel_fatal    );
}


async fn flywheel_system_set_motd(_in_motd : WasmAnyPtr, _motd_len : u32) -> WasmResult<()> {
    let motd : &str = todo!();
    trace!("Set server MOTD to {motd}");
    let text = Text::from_xml(motd, false, true);
    AsyncWorld.resource_scope::<ServerMotd, _>(|mut r_motd| { r_motd.0 = text; });
    Ok(())
}

async fn flywheel_trace(_in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    let msg : &str = todo!();
    trace!("{}", msg);
    Ok(())
}

async fn flywheel_debug(_in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    let msg : &str = todo!();
    debug!("{}", msg);
    Ok(())
}

async fn flywheel_info(_in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    let msg : &str = todo!();
    info!("{}", msg);
    Ok(())
}

async fn flywheel_pass(_in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    let msg : &str = todo!();
    pass!("{}", msg);
    Ok(())
}

async fn flywheel_warn(_in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    let msg : &str = todo!();
    warn!("{}", msg);
    Ok(())
}

async fn flywheel_error(_in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    let msg : &str = todo!();
    error!("{}", msg);
    Ok(())
}

async fn flywheel_fatal(_in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    let msg : &str = todo!();
    fatal!("{}", msg);
    Ok(())
}
