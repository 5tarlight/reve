use bevy::prelude::*;

use crate::{
    champion::{Champion, MyPlayer},
    constants::{MAX_MAP_X, MAX_MAP_Y, MIN_MAP_X, MIN_MAP_Y},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_map).add_system(move_mouse);
        // .add_system(grab_cursor);
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

fn move_mouse(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut player_query: Query<&Transform, (With<MyPlayer>, With<Champion>)>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    if buttons.pressed(MouseButton::Right) {
        let ptf = player_query.get_single_mut().unwrap();
        let ctf = camera_query.get_single().unwrap();
        let win = windows.get_primary().unwrap();
        let win_w = win.requested_width();
        let win_h = win.requested_height();

        // TODO : Right Click means not only move but attack.
        if let Some(pos) = win.cursor_position() {
            let gap_w = pos.x - (win_w / 2.);
            let gap_h = pos.y - (win_h / 2.);
            let target_w = ctf.translation.x + gap_w;
            let target_h = ctf.translation.y + gap_h;

            println!("Target : {}x{}", target_w, target_h);
            println!(
                "Distance : {}x{}",
                target_w - ptf.translation.x,
                target_h - ptf.translation.y
            );
        }
    }
}

// It causes cursor lock.
// fn grab_cursor(
//     mut windows: ResMut<Windows>,
//     btn: Res<Input<MouseButton>>,
//     key: Res<Input<KeyCode>>,
// ) {
//     let window = windows.get_primary_mut().unwrap();

//     if btn.just_pressed(MouseButton::Left) || btn.just_pressed(MouseButton::Right) {
//         // Confined not let cursor leave but let use cursor.
//         window.set_cursor_grab_mode(CursorGrabMode::Confined);

//         // window.set_cursor_visibility(false);
//     }

//     if key.just_pressed(KeyCode::Escape) {
//         window.set_cursor_grab_mode(CursorGrabMode::None);
//         window.set_cursor_visibility(true);
//     }
// }
