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
    #[arg(default_value = "stderr:debug")]
    pub output : Vec<SingleLogTarget>,

    /// Output colour behaviour [aliases: color]
    #[arg(short = 'c', long, alias = "color")]
    #[arg(default_value_t = ColourChoice::Auto)]
    pub colour : ColourChoice,

    /// The directory in which to run the server
    #[arg(short = 'd', long)]
    pub run_dir : PathBuf,

    /// Addresses to bind the server to
    #[arg(short = 'b', long)]
    #[arg(default_value = "127.0.0.1:25565,0.0.0.0:25565")]
    pub bind : SocketAddrs,

    /// Server list message, supports XML text
    #[arg(long)]
    #[arg(default_value = "<grey>A <gold>FlywheelMC</> Server</grey>")]
    pub motd : String

}
