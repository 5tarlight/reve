use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_map);
    }
}

fn move_map(key: Res<Input<KeyCode>>, mut cam_query: Query<&mut Transform, With<Camera2d>>) {
    if key.pressed(KeyCode::Left) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.x -= 20.;
    } else if key.pressed(KeyCode::Right) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.x += 20.;
    } else if key.pressed(KeyCode::Up) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.y += 20.;
    } else if key.pressed(KeyCode::Down) {
        let mut cam = cam_query.get_single_mut().unwrap();
        cam.translation.y -= 20.;
    }
}
