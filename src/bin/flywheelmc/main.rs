use flywheelmc_common::prelude::*;
use flywheelmc_players::{ FlywheelMcPlayersPlugin, MINECRAFT_VERSION };
use protocol::value::{
    Text,
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
            favicon            : Cow::Borrowed("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"),
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
