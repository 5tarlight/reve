use bevy::{
    prelude::{
        AssetServer, Camera2d, Camera2dBundle, Commands, Handle, Image, Plugin, Query, Res,
        StartupStage, Transform, Vec3, With,
    },
    sprite::SpriteBundle,
};

use crate::{
    champion::{Champion, Champions},
    constants::{GameInfo, Team, Textures, GAREN, MAP},
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup)
            .add_startup_system_to_stage(StartupStage::PostStartup, init_game);
    }
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
        "Garen" => Champions::Garen,
        _ => Champions::Garen,
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

fn init_game(
    mut commands: Commands,
    textures: Res<Textures>,
    mut cam_query: Query<&mut Transform, With<Camera2d>>,
    game_info: Res<GameInfo>,
) {
    // Spawn Map
    commands.spawn(SpriteBundle {
        texture: textures.map.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::splat(1.), // TODO: Camera zoom
            ..Default::default()
        },
        ..Default::default()
    });

    // Move camera location depends on team
    let mut cam = cam_query.get_single_mut().unwrap();
    match game_info.team {
        Team::Blue => {
            cam.translation.x = -4800.0;
            cam.translation.y = -4400.0;
        }
        Team::Red => {
            cam.translation.x = 5000.0;
            cam.translation.y = 3800.0;
        }
    }

    // Spawn Player champion
    let champ: Champion;
    let img: Handle<Image>;

    match game_info.champion {
        Champions::Garen => {
            champ = Champion {
                max_hp: 690.,
                hp: 690.,
                grow_hp: 98.,
                hp_gen: 8.,
                grow_hp_gen: 0.5,
                ad: 66.,
                grow_ad: 4.5,
                ap: 0.,
                grow_ap: 0.,
                attack_speed: 0.625,
                grow_attack_speed: 3.65,
                def_ad: 36.,
                grow_def_ad: 4.2,
                def_ap: 32.,
                grow_def_ap: 1.55,
                hit_range: 175.,
                move_speed: 340.,
                crit_prob: 0,
                cooldown: 0.,
                name: Champions::Garen,
                team: game_info.team.clone(),
            };
            img = textures.garen.clone();
        }
    }

    commands
        .spawn(SpriteBundle {
            texture: img,
            transform: Transform {
                scale: Vec3::splat(0.3),
                translation: Vec3::new(-4800.0, -4400., 2.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(champ);
}
