use bevy::prelude::{App, Commands, Component, Plugin, StartupStage};

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_minion)
            .add_system(spawn_minions);
    }
}

#[derive(Component)]
pub struct MinionManager;

#[derive(Component)]
pub struct MinionWaveCounter(u32);

fn init_minion(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(MinionManager)
        .insert(MinionWaveCounter(0));
}

fn spawn_minions() {}
