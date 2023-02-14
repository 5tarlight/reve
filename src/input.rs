use bevy::{
    prelude::{
        Camera2d, Commands, Component, Entity, Input, KeyCode, MouseButton, Plugin, Query, Res,
        Transform, Vec2, Vec3, With,
    },
    sprite::SpriteBundle,
    time::{Time, Timer, TimerMode},
    window::Windows,
};

use crate::{
    champion::{Champion, MyPlayer},
    constants::{Textures, CURSOR_DURATION, MAX_MAP_X, MAX_MAP_Y, MIN_MAP_X, MIN_MAP_Y},
    movement::MoveToPoint,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_map)
            .add_system(move_mouse)
            .add_system(cursor_spawn)
            .add_system(cursor_despawn);
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

#[derive(Component)]
pub struct CursorEffectSpawn(pub Vec3);

#[derive(Component)]
pub struct CursorEffectTimer(pub Timer);

impl Default for CursorEffectTimer {
    fn default() -> Self {
        // If texture atlas animation -> TimerMode::Repeat
        Self(Timer::from_seconds(CURSOR_DURATION, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct CursorEffect;

fn move_mouse(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut player_query: Query<(Entity, &Transform), (With<MyPlayer>, With<Champion>)>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    if buttons.pressed(MouseButton::Right) {
        let (entity, ptf) = player_query.get_single_mut().unwrap();
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

            if buttons.just_pressed(MouseButton::Right) {
                commands.spawn(CursorEffectSpawn(Vec3::new(target_w, target_h, 100.)));
            }

            commands
                .entity(entity)
                .insert(MoveToPoint(Vec2::new(target_w, target_h)));
        }
    }
}

fn cursor_spawn(
    mut commands: Commands,
    textures: Res<Textures>,
    query: Query<(Entity, &CursorEffectSpawn)>,
) {
    for (spawn_entity, spawn) in query.iter() {
        commands
            .spawn(SpriteBundle {
                texture: textures.cursor.clone(),
                transform: Transform {
                    translation: spawn.0,
                    scale: Vec3::splat(0.01),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(CursorEffect)
            .insert(CursorEffectTimer::default());
        commands.entity(spawn_entity).despawn();
    }
}

fn cursor_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CursorEffectTimer), With<CursorEffect>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
