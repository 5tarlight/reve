use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use reve::{input::InputPlugin, movement::MovementPlugin, setup::SetupPlugin, ui::ReveUiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Reve Of Legends".to_string(),
                width: 1920.0,
                height: 1080.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(SetupPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ReveUiPlugin)
        .run();
}
