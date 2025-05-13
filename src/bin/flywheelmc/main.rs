use flywheelmc_common::prelude::*;
use flywheelmc_players::{ FlywheelMcPlayersPlugin, MINECRAFT_VERSION };
use protocol::value::{
    Text, TextComponent, TextColour,
    Identifier,
    DimType, DimEffects, DimMonsterSpawnLightLevel
};


mod cli;


#[tokio::main]
async fn main() -> AppExit {
    let mut cli = cli::FlywheelMcCli::parse_check_colour();
    GLOBAL_FILTER.set(cli.log);
    LOG_TARGETS.extend(mem::replace(&mut cli.output, Vec::new()).into_iter().map(|t| t.0));

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlywheelMcPlayersPlugin {
            listen_addrs       : cli.bind,
            motd               : Text::from_xml(cli.motd, false, true),
            version            : Cow::Owned(format!("FlywheelMC {}", MINECRAFT_VERSION)),
            // TOOD: Favicon load file or accept b64.
            favicon            : Cow::Borrowed("iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAMAAACdt4HsAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAAA8UExURUdwTP748/////////////////////////+/Af8BAVMTaQgADDsPSSQFL/jSvtajp/9NTf/GTIV+iXJXgEo/9IwAAAAHdFJOUwD80BaaZz7Q6pnNAAAC0klEQVRYw91X2XKDMAysANscvoD//9dKvm1IGtOXTtWG6TTserWSbPj6+uMxYvwGPoEQMD3HD8LF8FQE4s15aiEeamCIXzGMEOxRAiD06kI/S2IScHqC45EEFGDWNUl44kAUsEqUwB+UQK9rltBdiFGkDCQVAsb+DNL6cj2h28acgaTf7hxSBtJ9JObwOANHcfbWYaoywIDOHIZYg4CXnb2EFhzJQEfQWchkQRTQa0K0IOHJBPbIghRdJmQLIlyprk7gQpwlXimpcCLfuMgmdudhhquLCRUEV6y+9YNQwOlTjwNumEVZJtq7Lx5WcKl0c0u5WQ8Cqj5BBeeaxPtL7SJueFAQIqDqE6SHI8JdbKp2EZPWNUE1LHSkaVXHhi6WSYOpCXTd61gHGZaOP6ooA50ZDcFe1QFPRVCbg+UoCJDfNgRzaSNVxWwxgoIiBRKwtAR7URYkM2orojERHbAXghld4InAJOQWmTKAk4ClrAIyzvMO6QxFB2S1Pkbe1MggSwRT6flMEiInLy0IFHlLQrxZlqWcDQTsnmFK9xwVw3ak79BhTGCxZeuRq0iAlQh3jS3DkfIL+MU0zU85kA1iGoNPYJIPuKUG/Eh4SwT1bDGfg9Pg7+Q0D+ZADnmY9F967PJ4W28AMQfnA7AxrIUBAHR1ukYGQT8JqLd5LOQeGGg5R86n+Jjn02JOlMfb9qTKEpwRSOElcwz/F8GD/BsBztwgwYugdVm4Z2Reiw7L3wjwW8w8ZwqI6mMekOF3Alwh9DzfcDgntbFLib87poaGAd3YKRnElmDXQ7enFHVfw+D6Qi9NGHjx5Eo26J8JLLw86fmV4UqA68PLg55D68OFgDqVv3/FgP01gdU/vX64lxS93xNYDR+8vtDAQKIoCBwcPnhMcW9akSMSWKPTUH5KQf3nG0n74fgUHmYvN7Lv5oF1Pq+PHGdwoGHCC+O/ef389/ENv2s5bHHprKEAAAAASUVORK5CYII="),
            compress_threshold : 64,
            mojauth_enabled    : true,
            server_id          : Cow::Borrowed("FLYWHEELSTANDALONE"),
            server_brand       : Cow::Borrowed("FlywheelMC"),
            default_dim_id     : Identifier::new_const("flywheelmc", "main"),
            default_dim_type   : DimType {
                fixed_time                      : Some(6000),
                has_skylight                    : true,
                has_ceiling                     : false,
                ultrawarm                       : false,
                natural                         : true,
                coordinate_scale                : 1.0,
                bed_works                       : true,
                respawn_anchor_works            : false,
                min_y                           : 0,
                height                          : 256,
                logical_height                  : 256,
                infiniburn                      : "#minecraft:infiniburn_overworld".to_string(),
                effects                         : DimEffects::Overworld,
                ambient_light                   : 0.0,
                piglin_safe                     : false,
                has_raids                       : true,
                monster_spawn_light_level       : DimMonsterSpawnLightLevel::Constant(0),
                monster_spawn_block_light_limit : 0
            },
            max_view_distance  : NonZeroU8::new(8).unwrap()
        })
        .run()
}
