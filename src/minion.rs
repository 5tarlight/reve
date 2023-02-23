use bevy::{
    prelude::{App, Commands, Component, Plugin, Query, StartupStage, Vec3, With},
    time::{Timer, TimerMode},
};

use crate::{
    constants::{
        BLUE_BOT_MINION_SPAWN, BLUE_MID_MINION_SPAWN, BLUE_TOP_MINION_SPAWN, MINION_PHASE_1,
        MINION_PHASE_2, MINION_SPAWN_GAP, MINION_SPAWN_START, RED_BOT_MINION_SPAWN,
        RED_MID_MINION_SPAWN, RED_TOP_MINION_SPAWN,
    },
    timer::GameTimer,
};

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_minion)
            .add_system(spawn_minions)
            .add_system(gen_minions);
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

#[derive(Debug, Clone, Copy)]
pub enum MinionType {
    Melee,
    Caster,
    Siege,
    Super,
}

#[derive(Component)]
pub struct SpawnMinionAfter(Timer, Vec3, MinionType);

impl SpawnMinionAfter {
    pub fn new(time: f32, loc: (f32, f32), mtype: MinionType) -> Self {
        let vec = Vec3::new(loc.0, loc.1, 3.);
        Self(
            Timer::from_seconds(time, TimerMode::Once),
            vec,
            mtype.clone(),
        )
    }
}

fn spawn_minions(
    mut commands: Commands,
    mut counter_query: Query<&mut MinionWaveCounter, With<MinionManager>>,
    timer_query: Query<&GameTimer>,
) {
    let time = timer_query.get_single().unwrap().0;
    let mut counter = counter_query.get_single_mut().unwrap();
    let booked = ((time - MINION_SPAWN_START) / MINION_SPAWN_GAP).floor() as i32;
    println!("{time} {booked}");

    if booked >= counter.0 as i32 {
        // Spawn new wave
        let mut siege = false;

        if time < MINION_PHASE_1 as f32 {
            if counter.0 % 3 == 0 {
                siege = true;
            }
        } else if time < MINION_PHASE_2 as f32 {
            if counter.0 % 2 == 0 {
                siege = true
            }
        } else {
            siege = true;
        }

        for i in 0..6 {
            for loc in [
                BLUE_TOP_MINION_SPAWN,
                BLUE_MID_MINION_SPAWN,
                BLUE_BOT_MINION_SPAWN,
            ] {
                if i == 2 && siege {
                    commands.spawn(SpawnMinionAfter::new(i as f32, loc, MinionType::Siege));
                } else if i < 3 {
                    if siege {
                        commands.spawn(SpawnMinionAfter::new(
                            (i + 1) as f32,
                            loc,
                            MinionType::Melee,
                        ));
                    } else {
                        commands.spawn(SpawnMinionAfter::new(i as f32, loc, MinionType::Melee));
                    }
                } else if i == 5 {
                    // TODO : Super minion gen
                } else {
                    if siege {
                        commands.spawn(SpawnMinionAfter::new(
                            (i + 1) as f32,
                            loc,
                            MinionType::Caster,
                        ));
                    } else {
                        commands.spawn(SpawnMinionAfter::new(i as f32, loc, MinionType::Caster));
                    }
                }
            }
            for loc in [
                RED_TOP_MINION_SPAWN,
                RED_MID_MINION_SPAWN,
                RED_BOT_MINION_SPAWN,
            ] {
                if i == 2 && siege {
                    commands.spawn(SpawnMinionAfter::new(i as f32, loc, MinionType::Siege));
                } else if i < 3 {
                    if siege {
                        commands.spawn(SpawnMinionAfter::new(
                            (i + 1) as f32,
                            loc,
                            MinionType::Melee,
                        ));
                    } else {
                        commands.spawn(SpawnMinionAfter::new(i as f32, loc, MinionType::Melee));
                    }
                } else if i == 5 {
                    // TODO Spawn Super minion
                } else {
                    if siege {
                        commands.spawn(SpawnMinionAfter::new(
                            (i + 1) as f32,
                            loc,
                            MinionType::Caster,
                        ));
                    } else {
                        commands.spawn(SpawnMinionAfter::new(i as f32, loc, MinionType::Caster));
                    }
                }
            }
        }
        counter.0 += 1;
    }
}

fn gen_minions() {}
