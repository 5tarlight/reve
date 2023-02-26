use crate::{
    champion::{Champion, Champions},
    constants::{
        GameInfo, Spells, Textures, PASSIVE_ICON_SIZE, SKILL_COOL_TEXT_SIZE, SKILL_ICON_SIZE,
        SKILL_UI_H, SKILL_UI_W, SPELL_ICON_SIZE,
    },
    skill::{
        SkillE, SkillP, SkillQ, SkillR, SkillStat, SkillStatus, SkillW, Spell, SpellD, SpellF,
    },
};
use bevy::{
    prelude::{
        BuildChildren, ButtonBundle, Color, Commands, Component, Entity, Handle, Image, NodeBundle,
        Plugin, Query, Res, StartupStage, TextBundle, With,
    },
    text::{Text, TextStyle},
    time::Time,
    ui::{
        AlignContent, AlignSelf, BackgroundColor, FlexDirection, JustifyContent, Size, Style,
        UiRect, Val,
    },
    window::Windows,
};

pub struct ReveUiPlugin;

impl Plugin for ReveUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_ui)
            .add_system(update_skill_ui_q);
    }
}

#[derive(Component)]
pub struct SkillUiParent;

/// Ui consumes percent of parent width.
/// Recommended for HP, MP.<br>
/// Value is 0 ~ 100
#[derive(Component)]
pub struct PercentWidth(f32);

fn init_ui(
    mut commands: Commands,
    textures: Res<Textures>,
    game_info: Res<GameInfo>,
    windows: Res<Windows>,
) {
    let windows = windows.get_primary().unwrap();
    let h = windows.height();

    commands
        .spawn(NodeBundle {
            // Root UI Parent
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SkillUiParent)
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    // Skill UI Parent
                    style: Style {
                        size: Size {
                            height: Val::Px(SKILL_UI_H),
                            width: Val::Px(SKILL_UI_W),
                        },
                        margin: UiRect::top(Val::Px(h - SKILL_UI_H)),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::Rgba {
                        red: 0.,
                        green: 0.1,  // ~= 28
                        blue: 0.027, // ~= 7
                        alpha: 1.,
                    }),
                    ..Default::default()
                })
                .add_children(|commands| {
                    // Skill UI Box
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            let skill_image = match game_info.champion {
                                Champions::GAREN => &textures.garen,
                                Champions::ASH => &textures.ash,
                            };
                            let [q, w, e, r] = Champion::get_skill_stats(game_info.champion);

                            // Passive
                            commands
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(PASSIVE_ICON_SIZE),
                                            Val::Px(PASSIVE_ICON_SIZE),
                                        ),
                                        align_self: AlignSelf::FlexEnd,
                                        margin: UiRect {
                                            left: Val::Px(5.),
                                            right: Val::Px(10.),
                                            top: Val::Px(20.),
                                            bottom: Val::Px(60.),
                                        },
                                        ..Default::default()
                                    },
                                    image: skill_image.p[0].clone().into(),
                                    ..Default::default()
                                })
                                .insert(SkillP);

                            // Q
                            commands
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(SKILL_ICON_SIZE),
                                            Val::Px(SKILL_ICON_SIZE),
                                        ),
                                        align_self: AlignSelf::FlexEnd,
                                        margin: UiRect {
                                            left: Val::Px(5.),
                                            right: Val::Px(5.),
                                            top: Val::Px(20.),
                                            bottom: Val::Px(60.),
                                        },
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    image: skill_image.q[0].clone().into(),
                                    ..Default::default()
                                })
                                .insert(SkillQ)
                                .insert(q);

                            // W
                            commands
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(SKILL_ICON_SIZE),
                                            Val::Px(SKILL_ICON_SIZE),
                                        ),
                                        align_self: AlignSelf::FlexEnd,
                                        margin: UiRect {
                                            left: Val::Px(5.),
                                            right: Val::Px(5.),
                                            top: Val::Px(20.),
                                            bottom: Val::Px(60.),
                                        },
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    image: skill_image.w[0].clone().into(),
                                    ..Default::default()
                                })
                                .insert(SkillW)
                                .insert(w);

                            // E
                            commands
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(SKILL_ICON_SIZE),
                                            Val::Px(SKILL_ICON_SIZE),
                                        ),
                                        align_self: AlignSelf::FlexEnd,
                                        margin: UiRect {
                                            left: Val::Px(5.),
                                            right: Val::Px(5.),
                                            top: Val::Px(20.),
                                            bottom: Val::Px(60.),
                                        },
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    image: skill_image.e[0].clone().into(),
                                    ..Default::default()
                                })
                                .insert(SkillE)
                                .insert(e);

                            // R
                            commands
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(SKILL_ICON_SIZE),
                                            Val::Px(SKILL_ICON_SIZE),
                                        ),
                                        align_self: AlignSelf::FlexEnd,
                                        margin: UiRect {
                                            left: Val::Px(5.),
                                            right: Val::Px(5.),
                                            top: Val::Px(20.),
                                            bottom: Val::Px(60.),
                                        },
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    image: skill_image.r[0].clone().into(),
                                    ..Default::default()
                                })
                                .insert(SkillR)
                                .insert(r);

                            fn get_spell_img(
                                spell: Spells,
                                textures: &Res<Textures>,
                            ) -> Handle<Image> {
                                match spell {
                                    Spells::BARRIER => textures.spell.barrier.clone(),
                                    Spells::CLARITY => textures.spell.clarity.clone(),
                                    Spells::CLEANSE => textures.spell.cleanse.clone(),
                                    Spells::EXHAUST => textures.spell.exhaust.clone(),
                                    Spells::FLASH => textures.spell.flash.clone(),
                                    Spells::GHOST => textures.spell.ghost.clone(),
                                    Spells::HEAL => textures.spell.heal.clone(),
                                    Spells::IGNITE => textures.spell.ignite.clone(),
                                    Spells::MARK => textures.spell.mark.clone(),
                                    Spells::SMITE => textures.spell.smite.clone(),
                                    Spells::TELEPORT => textures.spell.teleport.clone(),
                                }
                            }

                            let icon_d = get_spell_img(game_info.spell_d, &textures);
                            let icon_f = get_spell_img(game_info.spell_f, &textures);
                            let stat_d = Spell::get_spell_stat(game_info.spell_d);
                            let stat_f = Spell::get_spell_stat(game_info.spell_f);

                            // D
                            commands
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(SPELL_ICON_SIZE),
                                            Val::Px(SPELL_ICON_SIZE),
                                        ),
                                        align_self: AlignSelf::FlexEnd,
                                        margin: UiRect {
                                            left: Val::Px(15.),
                                            right: Val::Px(5.),
                                            top: Val::Px(20.),
                                            bottom: Val::Px(60.),
                                        },
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    image: icon_d.into(),
                                    ..Default::default()
                                })
                                .insert(SpellD)
                                .insert(stat_d);

                            // F
                            commands
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(SPELL_ICON_SIZE),
                                            Val::Px(SPELL_ICON_SIZE),
                                        ),
                                        align_self: AlignSelf::FlexEnd,
                                        margin: UiRect {
                                            left: Val::Px(5.),
                                            right: Val::Px(5.),
                                            top: Val::Px(20.),
                                            bottom: Val::Px(60.),
                                        },
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    image: icon_f.into(),
                                    ..Default::default()
                                })
                                .insert(SpellF)
                                .insert(stat_f);
                        });

                    // Health UI Box
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                size: Size {
                                    width: Val::Percent(80.),
                                    height: Val::Auto,
                                },
                                margin: UiRect::top(Val::Px(-35.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            // HP
                            commands.spawn(NodeBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Percent(100.),
                                        height: Val::Px(15.),
                                    },
                                    margin: UiRect::left(Val::Px(SKILL_UI_W * 0.2 / 2.)),
                                    ..Default::default()
                                },
                                background_color: BackgroundColor(Color::rgba_u8(0, 255, 0, 255)),
                                ..Default::default()
                            });

                            // MP
                            commands.spawn(NodeBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Percent(100.),
                                        height: Val::Px(15.),
                                    },
                                    margin: UiRect::left(Val::Px(SKILL_UI_W * 0.2 / 2.)),
                                    ..Default::default()
                                },
                                background_color: BackgroundColor(Color::rgba_u8(0, 0, 255, 255)),
                                ..Default::default()
                            });
                        });
                });
        });
}

#[derive(Component)]
pub struct CooldodwnTextQ;

#[derive(Component)]
pub struct CooldodwnTextW;

#[derive(Component)]
pub struct CooldodwnTextE;

#[derive(Component)]
pub struct CooldodwnTextR;

fn update_skill_ui_q(
    mut commands: Commands,
    mut skill_query: Query<(Entity, &mut SkillStat), With<SkillQ>>,
    textures: Res<Textures>,
    mut text_query: Query<&mut Text, With<CooldodwnTextQ>>,
    time: Res<Time>,
) {
    let d = time.delta().as_secs_f32();

    for (entity, mut stat) in skill_query.iter_mut() {
        match stat.status {
            SkillStatus::Available => {
                if let Ok(mut text) = text_query.get_single_mut() {
                    text.sections[0].value = format!("");
                }
            }
            SkillStatus::Cooldown(cool) => {
                // Display cooldown
                let cool_str = (if cool < 10. {
                    (cool * 10.).ceil() / 10.
                } else {
                    cool.ceil()
                })
                .to_string();

                commands.entity(entity).with_children(|parent| {
                    if let Ok(mut text) = text_query.get_single_mut() {
                        text.sections[0].value = cool_str;
                    } else {
                        parent
                            .spawn(
                                TextBundle::from_section(
                                    cool_str,
                                    TextStyle {
                                        font: textures.rix_font.clone(),
                                        font_size: SKILL_COOL_TEXT_SIZE,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    align_self: AlignSelf::Center,
                                    border: UiRect::all(Val::Px(1.)),
                                    ..Default::default()
                                }),
                            )
                            .insert(CooldodwnTextQ)
                            .insert(CooldodwnTextQ);
                    }
                });

                // Update cooldown
                let left = 0.0f32.max(cool - d);

                if left > 0. {
                    stat.status = SkillStatus::Cooldown(left);
                } else {
                    stat.status = SkillStatus::Available;
                }
            }
            SkillStatus::Disabled => {}
            SkillStatus::Active => {}
            SkillStatus::NoMp => {}
            SkillStatus::NotHave => {}
            SkillStatus::StandBy => {}
        }
    }
}
