use bevy::{
    prelude::{Camera2d, Commands, Plugin, Query, Res, StartupStage, Transform, Vec3, With},
    sprite::SpriteBundle,
};

use crate::constants::{GameInfo, Team, Textures};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_game);
    }
}

fn init_game(
    mut commands: Commands,
    textures: Res<Textures>,
    mut cam_query: Query<&mut Transform, With<Camera2d>>,
    game_info: Res<GameInfo>,
) {
    commands.spawn(SpriteBundle {
        texture: textures.map.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::splat(1.), // TODO: Camera zoom
            ..Default::default()
        },
        ..Default::default()
    });

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
}
