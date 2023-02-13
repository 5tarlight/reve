use bevy::prelude::*;

use crate::constants::{MAX_MAP_X, MAX_MAP_Y, MIN_MAP_X, MIN_MAP_Y};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_map);
    }
}

fn move_map(key: Res<Input<KeyCode>>, mut cam_query: Query<&mut Transform, With<Camera2d>>) {
    // TODO : Movement speed multiplier

    if key.pressed(KeyCode::Left) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.x = MIN_MAP_X.max(cam.translation.x - 20.);
    } else if key.pressed(KeyCode::Right) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.x = MAX_MAP_X.min(cam.translation.x + 20.);
    } else if key.pressed(KeyCode::Up) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.y = MAX_MAP_Y.min(cam.translation.y + 20.);
    } else if key.pressed(KeyCode::Down) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.y = MIN_MAP_Y.max(cam.translation.y - 20.);
    }
}
