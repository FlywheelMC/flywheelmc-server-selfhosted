#![feature(
    // Standard Library
    ip
)]


use flywheelmc_common::prelude::*;
use flywheelmc_players::{ FlywheelMcPlayersPlugin, RejectNewConns, MINECRAFT_VERSION };
use flywheelmc_players::player::{ PlayerJoined, Player, KickPlayer };
use flywheelmc_wasm::{ FlywheelMcWasmPlugin, WasmGlobals };
use flywheelmc_wasm::runner::{ StartWasm, WasmStartedEvent, WasmErrorEvent, WasmRunnerInstance };
use flywheelmc_wasm::runner::player::PlayerBindWasm;
use protocol::value::{
    Text, TextComponent, TextColour,
    Identifier,
    DimType, DimEffects, DimMonsterSpawnLightLevel
};


mod cli;

mod import_defs;


fn main() -> AppExit {
    let mut cli = cli::FlywheelMcCli::parse_check_colour();
    GLOBAL_FILTER.set(cli.log);
    LOG_TARGETS.extend(mem::take(&mut cli.output).into_iter().map(|t| t.0));

    let favicon = cli.favicon.map_or(
        Cow::Borrowed("iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAMAAACdt4HsAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAABsUExURUdwTAAAAAAAAHdcTXNWSAAAAAAAAItqWzkmHQcEA5x3ZFY8LlQ4LHBRQWQ9L35hUEg4L2s6KnE/LmBLQFdEOoFjUotNOmdTR4BINk89M3lDMzwzLz49PJlVQG5HOXxRQatfSFo2KignJmNgXyXMkvwAAAARdFJOUwABCv7+Jhf+Yz3+mMrV58HrcyjiewAABM1JREFUWMPtV9lW6zgQhEi2bGchktpavSb//49TLScsl22YO4+IQwLEVaquXiQeHn7X7/puPW7rL6Av679Cd1g/YHgLreuqatumre8cj/+SYFewgO73h5Mxh31TVVW9+57hvu0NmrTRWhte+8NhX92C+Qq/A5ahJ81Yw994TfyT0U3btvVXDIx/mp25Q5NzbnQuGMcsemLa5otAGL8XISXejbFMgFdtFrzNEwV8cLg5+iG+rhoxBQBSwL4prCaEEQL0Ah3K0zSzrFPTVB8xAN8cHJuGpwrBmnOCiE3BLIWfx6CLIx8xlPgl6y37AzdRHGII4JymUU2zJFoWfIqUQET9J8Pjrm6eehemieEJAqaeLM3MJCh1YpYeBM44qNB0bv9g4ABOIz6apjk47BvcpHpxvVjnQGR72c+zXpagw8ge0+FtFCUAMTqEKGfZW/gVgur8NWeaO2+tlF4GmJmQ2wVGEb0NAvhG9Y49nOZeeQ/1QXrr8zCQtGTBYS2RTdqNy/hOAgfgIxVxBgSi79WGiszgfc+/IBJhkKBlSVqvryUggPpJEYgTJ9CpHgQKz3tvh2G4XPIgyEYrhTBpdMvIZtGhfZZQHOjnhXMYQkLOQcCRU74MmRnwlgdL0Yc0TtMELyFhX78Q1O2RpgUe6hSIWGvf9cLThXe/r0wZ3tgY12XcUnmTgN6onvB3B2JjZi8QLVklaXgG50u2ka7+eraeYuZEGEiotrZCBG0nKK6OiWc4KIW0V+L4c4Z8vOYsOA3Wyx5JcWyjjncJhaADsUGtmwn+CXG9XrJHEURAL8UFpMFbIYQSFlrd62oqIXR9ZGkgPh8EnkUtIAfFBY6EtpzI8tc0liEV4j2VXAZdh5xr1KJZI53P/upp8LLItiDIFomUqFFrz6dxKeMGdh82F8ok6XroTSwBaTh7Lj3ReY8WsiwhIi4BezgNU3E76Wj9GxcgYWUJGqWnelAglSDwhHKioe8Z38FfMc+zKUqtfCZ4hARVJPD4E6gkSV4wRolIfrgMLAYE4FEz5ptJCQKOt0QWCai9TQIaSnL0ECFRUTJniwogYgPQ4lIGntHJRDq9TAWeJyWVAfElU5zoEQB35OCtul4Hq7ifuGN0mfkoilcdyRKeOklcpoEHuyHbs20SHnqFfJJXHdfxuA3dkKMU91J8eEklRcxxfGEH62EFDwayHSqjtIfaZmIKweToxeuhUjqSJUxOh7WUpMV8NHhDBMgnhtRszHbUORboTzyc307FrvOYvVFnN+JYCNvxNj9Dy4mFhnc6R+y//2AsdkJxM2bEsGrYVU6Z7aQz3Opl4Du3ElIrm/eDuVVCyV5BBE6VdRx5OnPONihmjdkMIBhy5FPy/dnUHBGwwAhGG+JM0iEtZdewhb/yiqigvt9/cjo1I8yWSFykaV2zYR3YmKE8FRB8RK9IcXx/Nt0ZGiXkjJnAZlCMKZg3WMwUVEdT1Z+c0LigtM1xVAq9q7j/QQIaxmIgoJ14iX21+/SOwDO+GcPxFLhzuJxRS8/QDf/0Kf5GUeMu04x8PqCX1CusOB73+7b9Cn8TscOIwcMugOAFiktSVW9Xvu/vejtcEatmfxx7+Qq6+/FtsWXotu3uh7fe53vu7v+5Mv/dlf33357f9e36B75tiuIj3qHZAAAAAElFTkSuQmCC"),
        Cow::Owned
    );

    if (cli.noauth) {
        warn!("==============================================================================");
        warn!("                            RUNNING IN NOAUTH MODE");
        warn!("");
        warn!("The server will not attempt to authenticate players");
        warn!("Anyone can join with any username, potentially stealing each other's save-data");
        warn!("Do not run a publically available server with noauth");
        warn!("==============================================================================");
        if (cli.bind.iter().any(|addr| addr.ip().is_global())) {
            error!("Globally routable bind addresses are forbidden in noauth mode");
            return AppExit::error();
        }
        warn!("Server will start in 3 seconds...");
        std::thread::sleep(Duration::from_secs(3));
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlywheelMcPlayersPlugin {
            max_conns          : cli.max_conns,
            listen_addrs       : cli.bind,
            motd               : Text::from_xml(cli.motd, false, true),
            version            : Cow::Owned(format!("FlywheelMC (Selfhosted) {MINECRAFT_VERSION}")),
            favicon,
            compress_threshold : 64,
            mojauth_enabled    : ! cli.noauth,
            server_id          : Cow::Borrowed("FLYWHEELSELFHOSTED"),
            server_brand       : Cow::Borrowed("FlywheelMC (Selfhosted)"),
            kick_footer        : Text::from(vec![
                TextComponent::of_literal("ꜰʟʏᴡʜᴇᴇʟ").colour(TextColour::RGB(64, 196, 138)),
                TextComponent::of_literal("ᴍᴄ").colour(TextColour::RGB(212, 112, 80))
            ]),
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
            max_view_distance  : NonZeroU8::new(32).unwrap()
        })
        .add_plugins({
            let mut wp = FlywheelMcWasmPlugin::default();
            import_defs::define_all(&mut wp.import_funcs);
            wp
        })
        .insert_resource(SourceWasmFile(cli.source_wasm))
        .add_systems(Startup, start_game)
        .add_systems(Update, handle_game_events)
        .add_systems(Update, join_players)
        .run()
}


#[derive(Resource)]
struct SourceWasmFile(PathBuf);


fn start_game(
        r_wasm   : Res<WasmGlobals>,
        r_source : Res<SourceWasmFile>,
    mut ew_start : EventWriter<StartWasm>
) { ew_start.write(r_wasm.new_from_file(&r_source.0)); }


fn handle_game_events(
    mut cmds       : Commands,
    mut ew_exit    : EventWriter<AppExit>,
    mut er_started : EventReader<WasmStartedEvent>,
    mut er_error   : EventReader<WasmErrorEvent>
) {
    for WasmStartedEvent { .. } in er_started.read() {
        cmds.remove_resource::<RejectNewConns>();
        pass!("WASM runner started");
    }
    for WasmErrorEvent { err, .. } in er_error.read() {
        fatal!("WASM runner encountered an error: {:?}", err);
        ew_exit.write(AppExit::error());
    }
}


static NOT_READY_TEXT : LazyLock<Text> = LazyLock::new(|| Text::from_xml("<#d36d4f>Server is still starting.</>", true, true));

fn join_players(
    mut q_runner  : Query<(Entity,), (With<WasmRunnerInstance>,)>,
        q_players : Query<(&Player,)>,
    mut er_join   : EventReader<PlayerJoined>,
    mut ew_bind   : EventWriter<PlayerBindWasm>,
    mut ew_kick   : EventWriter<KickPlayer>
) {
    if (! er_join.is_empty()) {
        if let Ok((runner_entity,)) = q_runner.single_mut() {
            for PlayerJoined { entity : player_entity, .. } in er_join.read() {
                ew_bind.write(PlayerBindWasm {
                    player : *player_entity,
                    runner : runner_entity
                });
            }
        } else {
            let message = &*NOT_READY_TEXT;
            for PlayerJoined { entity, .. } in er_join.read() {
                if let Ok((player,)) = q_players.get(*entity) {
                    warn!("Player {} ({}) tried to join while WASM runner is still starting", player.username(), player.uuid());
                }
                ew_kick.write(KickPlayer {
                    entity  : *entity,
                    message : message.clone()
                });
            }
        }
    }
}
