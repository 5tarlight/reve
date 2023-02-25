use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use reve::{
    entity::DamageSystem, input::InputPlugin, minion::MinionPlugin, movement::MovementPlugin,
    setup::SetupPlugin, timer::TimerPlugin, ui::ReveUiPlugin,
};

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
        .add_plugin(TimerPlugin)
        .add_plugin(MinionPlugin)
        .add_plugin(DamageSystem)
        .run();
}
