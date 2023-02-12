use bevy::prelude::*;
use reve::constants::{Champion, GameInfo, Team, Textures, GAREN, MAP};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "REVE Online".to_string(),
                width: 1920.0,
                height: 1080.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, init_game)
        .run();
}

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load args
    let args = std::env::args().collect::<Vec<String>>();
    dbg!(&args);

    let username = args[1].clone();
    let token = args[2].clone();
    let server = args[3].clone();
    let room = args[4].clone();
    let team = args[5].clone();
    let champion = args[6].clone();

    let champion = match champion.as_str() {
        "Garen" => Champion::Garen,
        _ => Champion::Garen,
    };

    let game_info = GameInfo {
        username,
        token,
        server,
        room,
        team: team.eq("red").then(|| Team::Red).unwrap_or(Team::Blue),
        champion,
    };

    commands.insert_resource(game_info);

    // Load assets
    let textures = Textures {
        map: asset.load(MAP),
        garen: asset.load(GAREN),
    };
    commands.insert_resource(textures);
}

fn init_game(mut commands: Commands, textures: Res<Textures>) {
    commands.spawn(SpriteBundle {
        texture: textures.map.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::splat(1.), // TODO: Camera zoom
            ..Default::default()
        },
        ..Default::default()
    });
}
