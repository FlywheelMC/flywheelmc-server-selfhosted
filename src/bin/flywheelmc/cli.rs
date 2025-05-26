use flywheelmc_common::prelude::*;


#[derive(Debug, Parser)]
#[command(
    name       = "FlywheelMC Selfhosted Server",
    version,
    author,
    about,
    long_about = None,
    help_template = concat!(
        "{before-help}",
        "\x1b[1m\x1b[4mFlywheelMC Selfhosted Server\x1b[0m",
        " (\x1b[1m\x1b[4m", env!("CARGO_PKG_VERSION"), "\x1b[0m)\n",
        "{tab}Licensed under ", env!("CARGO_PKG_LICENSE"), "\n",
        "{tab}", env!("CARGO_PKG_HOMEPAGE"), "\n\n",
        "{usage-heading} {usage}\n\n",
        "{all-args}",
        "{after-help}"
    ),
    arg_required_else_help = true,
    disable_version_flag   = true
)]
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

    /// Maximum number of connections. No maximum if omitted
    #[arg(long)]
    pub max_conns : Option<usize>,

    /// Server list message, supports XML text
    #[arg(long)]
    #[arg(default_value = "<#7e7e7e>A <#d36d4f>FlywheelMC</> Server</>")]
    pub motd : String,

    /// Server list favicon, file path or b64
    #[arg(long)]
    pub favicon : Option<String>, // TODO: Load favicon from file if file path.

    /// Disables Mojang account authentication. **NOT RECOMMENDED**
    #[arg(long)]
    pub noauth : bool,

    /// Source server WASM file.
    pub source_wasm : PathBuf,

    // // TODO: CLI source wasm watch
    // /// Restart the server if the WASM file changed.
    // #[arg(long)]
    // pub watch : bool

}
