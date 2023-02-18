use bevy::prelude::{Commands, Plugin, StartupStage};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_ui);
    }
}

fn init_ui(mut commands: Commands) {}
