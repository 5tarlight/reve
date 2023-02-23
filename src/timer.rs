use bevy::{
    prelude::{Commands, Component, Plugin, Query, Res, StartupStage},
    time::Time,
};

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_timer)
            .add_system(update_timer);
    }
}

#[derive(Component)]
pub struct TimeManager;

#[derive(Component)]
pub struct GameTimer(f32);

impl GameTimer {
    pub fn as_secs(m: u32, s: u32) -> f32 {
        let secs = m * 60 + s;
        secs as f32
    }
}

fn spawn_timer(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(TimeManager)
        .insert(GameTimer(0.));
}

fn update_timer(mut timer_query: Query<&mut GameTimer>, time: Res<Time>) {
    let d = time.delta().as_secs_f32();
    let mut timer = timer_query.get_single_mut().unwrap();
    timer.0 += d;
}
