use flywheelmc_common::prelude::*;
use flywheelmc_players::ServerMotd;
use flywheelmc_wasm::{ ImportFuncs, WasmCallCtx };
use flywheelmc_wasm::{ WasmAnyPtr, WasmResult };
use protocol::value::Text;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    import_funcs.define("flywheel_system_set_motd", flywheel_system_set_motd );
    import_funcs.define("flywheel_trace",    flywheel_trace    );
    import_funcs.define("flywheel_debug",    flywheel_debug    );
    import_funcs.define("flywheel_info",     flywheel_info     );
    import_funcs.define("flywheel_pass",     flywheel_pass     );
    import_funcs.define("flywheel_warn",     flywheel_warn     );
    import_funcs.define("flywheel_error",    flywheel_error    );
    import_funcs.define("flywheel_fatal",    flywheel_fatal    );
}


async fn flywheel_system_set_motd(
    ctx       : WasmCallCtx<'_>,
    in_motd  : WasmAnyPtr,
    motd_len : u32
) -> WasmResult<()> {
    let motd = ctx.mem_read_str(in_motd, motd_len)?;
    let text = Text::from_xml(motd, false, true);
    trace!("Set server MOTD to {:?}", text.to_string());
    AsyncWorld.resource_scope::<ServerMotd, _>(|mut r_motd| { r_motd.0 = text; });
    Ok(())
}

async fn flywheel_trace(
    ctx     : WasmCallCtx<'_>,
    in_msg  : WasmAnyPtr,
    msg_len : u32
) -> WasmResult<()> {
    let msg = ctx.mem_read_str(in_msg, msg_len)?;
    trace!("{}", msg);
    Ok(())
}

async fn flywheel_debug(
    ctx     : WasmCallCtx<'_>,
    in_msg  : WasmAnyPtr,
    msg_len : u32
) -> WasmResult<()> {
    let msg = ctx.mem_read_str(in_msg, msg_len)?;
    debug!("{}", msg);
    Ok(())
}

async fn flywheel_info(
    ctx     : WasmCallCtx<'_>,
    in_msg  : WasmAnyPtr,
    msg_len : u32
) -> WasmResult<()> {
    let msg = ctx.mem_read_str(in_msg, msg_len)?;
    info!("{}", msg);
    Ok(())
}

async fn flywheel_pass(
    ctx     : WasmCallCtx<'_>,
    in_msg  : WasmAnyPtr,
    msg_len : u32
) -> WasmResult<()> {
    let msg = ctx.mem_read_str(in_msg, msg_len)?;
    pass!("{}", msg);
    Ok(())
}

async fn flywheel_warn(
    ctx     : WasmCallCtx<'_>,
    in_msg  : WasmAnyPtr,
    msg_len : u32
) -> WasmResult<()> {
    let msg = ctx.mem_read_str(in_msg, msg_len)?;
    warn!("{}", msg);
    Ok(())
}

async fn flywheel_error(
    ctx     : WasmCallCtx<'_>,
    in_msg  : WasmAnyPtr,
    msg_len : u32
) -> WasmResult<()> {
    let msg = ctx.mem_read_str(in_msg, msg_len)?;
    error!("{}", msg);
    Ok(())
}

async fn flywheel_fatal(
    ctx     : WasmCallCtx<'_>,
    in_msg  : WasmAnyPtr,
    msg_len : u32
) -> WasmResult<()> {
    let msg = ctx.mem_read_str(in_msg, msg_len)?;
    fatal!("{}", msg);
    Ok(())
}
