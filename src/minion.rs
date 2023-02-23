use bevy::{
    prelude::{
        App, Commands, Component, Entity, Plugin, Query, Res, StartupStage, Transform, Vec2, Vec3,
        With,
    },
    sprite::SpriteBundle,
    time::{Time, Timer, TimerMode},
};

use crate::{
    constants::{
        Team, Textures, BLUE_BOT_MINION_SPAWN, BLUE_MID_MINION_SPAWN, BLUE_TOP_MINION_SPAWN,
        MINION_PHASE_1, MINION_PHASE_2, MINION_SCALE, MINION_SPAWN_GAP, MINION_SPAWN_START,
        MINION_VEL, RED_BOT_MINION_SPAWN, RED_MID_MINION_SPAWN, RED_TOP_MINION_SPAWN,
        SUPER_MINION_SCALE,
    },
    movement::{MoveToPoint, Velocity},
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MinionType {
    Melee,
    Caster,
    Siege,
    Super,
}

#[derive(Component)]
pub struct SpawnMinionAfter(Timer, Vec3, MinionType, Team);

impl SpawnMinionAfter {
    pub fn new(time: f32, loc: (f32, f32), mtype: MinionType, team: Team) -> Self {
        let vec = Vec3::new(loc.0, loc.1, 3.);
        Self(
            Timer::from_seconds(time, TimerMode::Once),
            vec,
            mtype.clone(),
            team.clone(),
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

    // println!("{time} {booked}");

    if booked >= counter.0 as i32 {
        // Spawn new wave

        counter.0 += 1;

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

        println!("Wave spawn({}), Siege? {}", counter.0, siege);

        for i in 0..6 {
            for loc in [
                BLUE_TOP_MINION_SPAWN,
                BLUE_MID_MINION_SPAWN,
                BLUE_BOT_MINION_SPAWN,
            ] {
                if i == 2 && siege {
                    commands.spawn(SpawnMinionAfter::new(
                        i as f32,
                        loc,
                        MinionType::Melee,
                        Team::Blue,
                    ));

                    commands.spawn(SpawnMinionAfter::new(
                        (i + 1) as f32,
                        loc,
                        MinionType::Siege,
                        Team::Blue,
                    ));
                } else if i > 2 {
                    if siege {
                        commands.spawn(SpawnMinionAfter::new(
                            (i + 1) as f32,
                            loc,
                            MinionType::Caster,
                            Team::Blue,
                        ));
                    } else {
                        commands.spawn(SpawnMinionAfter::new(
                            i as f32,
                            loc,
                            MinionType::Caster,
                            Team::Blue,
                        ));
                    }
                } else if i == 5 {
                    if siege {
                        commands.spawn(SpawnMinionAfter::new(
                            (i + 1) as f32,
                            loc,
                            MinionType::Caster,
                            Team::Blue,
                        ));
                    }

                    // TODO : Super minion gen
                } else {
                    commands.spawn(SpawnMinionAfter::new(
                        i as f32,
                        loc,
                        MinionType::Melee,
                        Team::Blue,
                    ));
                }
            }

            for loc in [
                RED_TOP_MINION_SPAWN,
                RED_MID_MINION_SPAWN,
                RED_BOT_MINION_SPAWN,
            ] {
                if i == 2 && siege {
                    commands.spawn(SpawnMinionAfter::new(
                        i as f32,
                        loc,
                        MinionType::Melee,
                        Team::Red,
                    ));

                    commands.spawn(SpawnMinionAfter::new(
                        (i + 1) as f32,
                        loc,
                        MinionType::Siege,
                        Team::Red,
                    ));
                } else if i > 2 {
                    if siege {
                        commands.spawn(SpawnMinionAfter::new(
                            (i + 1) as f32,
                            loc,
                            MinionType::Caster,
                            Team::Red,
                        ));
                    } else {
                        commands.spawn(SpawnMinionAfter::new(
                            i as f32,
                            loc,
                            MinionType::Caster,
                            Team::Red,
                        ));
                    }
                } else if i == 5 {
                    commands.spawn(SpawnMinionAfter::new(
                        (i + 1) as f32,
                        loc,
                        MinionType::Caster,
                        Team::Red,
                    ));
                    // TODO Spawn Super minion
                } else {
                    commands.spawn(SpawnMinionAfter::new(
                        i as f32,
                        loc,
                        MinionType::Melee,
                        Team::Red,
                    ));
                }
            }
        }
    }
}

fn gen_minions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnMinionAfter)>,
    texture: Res<Textures>,
) {
    for (entity, mut after) in query.iter_mut() {
        after.0.tick(time.delta());

        if after.0.finished() {
            let item = match after.3 {
                Team::Blue => match after.2 {
                    MinionType::Melee => texture.mob.blue_melee_minion.clone(),
                    MinionType::Caster => texture.mob.blue_caster_minion.clone(),
                    MinionType::Siege => texture.mob.blue_siege_minion.clone(),
                    MinionType::Super => texture.mob.blue_super_minion.clone(),
                },
                Team::Red => match after.2 {
                    MinionType::Melee => texture.mob.red_melee_minion.clone(),
                    MinionType::Caster => texture.mob.red_caster_minion.clone(),
                    MinionType::Siege => texture.mob.red_siege_minion.clone(),
                    MinionType::Super => texture.mob.red_super_minion.clone(),
                },
            };

            let scale = if after.2 == MinionType::Super {
                SUPER_MINION_SCALE
            } else {
                MINION_SCALE
            };

            commands
                .spawn(SpriteBundle {
                    texture: item,
                    transform: Transform {
                        scale: Vec3::splat(scale),
                        translation: after.1,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity(MINION_VEL))
                .insert(MoveToPoint(Vec2::new(0., 0.)));

            commands.entity(entity).despawn();
        }
    }
}
