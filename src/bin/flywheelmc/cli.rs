use flywheelmc_common::prelude::*;


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct FlywheelMcCli {

    /// Default global logging levels [example: `info,flywheelmc_players::conn=debug`]
    #[arg(short = 'l', long)]
    #[arg(default_value = "debug")]
    pub log : LevelFilter,

    /// Logging targets [example: `stderr:trace,flywheelmc_players::conn=debug`, `./logs/errors.txt:error`]
    #[arg(short = 'o', long)]
    #[arg(default_value = "stderr")]
    pub output : Vec<SingleLogTarget>,

    /// Output colour behaviour [aliases: color]
    #[arg(short = 'c', long, alias = "color")]
    #[arg(default_value_t = ColourChoice::Auto)]
    pub colour : ColourChoice,

    /// Addresses to bind the server to
    #[arg(short = 'b', long)]
    #[arg(default_value = "127.0.0.1:25565,0.0.0.0:25565")]
    pub bind : SocketAddrs,

    /// Server list message, supports XML text
    #[arg(long)]
    #[arg(default_value = "<#7e7e7e>A <#41c389>FlywheelMC</> Server</>")]
    pub motd : String,

    /// Source server WASM file.
    pub source_wasm : PathBuf,

    // // TODO: CLI source wasm watch
    // /// Restart the server if the WASM file changed.
    // #[arg(long)]
    // pub watch : bool

}
