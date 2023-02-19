use bevy::{
    prelude::{
        BuildChildren, ButtonBundle, Color, Commands, Component, Handle, Image, NodeBundle, Plugin,
        Res, StartupStage,
    },
    ui::{AlignSelf, BackgroundColor, JustifyContent, Size, Style, UiRect, Val},
    window::Windows,
};

use crate::{
    champion::{Champion, Champions},
    constants::{
        GameInfo, Spell, Textures, PASSIVE_ICON_SIZE, SKILL_ICON_SIZE, SKILL_UI_H, SKILL_UI_W,
        SPELL_ICON_SIZE,
    },
    skill::{SkillE, SkillP, SkillQ, SkillR, SkillW, SpellD, SpellF},
};

pub struct ReveUiPlugin;

impl Plugin for ReveUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_ui);
    }
}

#[derive(Component)]
pub struct SkillUiParent;

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
                                size: Size::new(Val::Px(SKILL_ICON_SIZE), Val::Px(SKILL_ICON_SIZE)),
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect {
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    top: Val::Px(20.),
                                    bottom: Val::Px(60.),
                                },
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
                                size: Size::new(Val::Px(SKILL_ICON_SIZE), Val::Px(SKILL_ICON_SIZE)),
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect {
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    top: Val::Px(20.),
                                    bottom: Val::Px(60.),
                                },
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
                                size: Size::new(Val::Px(SKILL_ICON_SIZE), Val::Px(SKILL_ICON_SIZE)),
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect {
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    top: Val::Px(20.),
                                    bottom: Val::Px(60.),
                                },
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
                                size: Size::new(Val::Px(SKILL_ICON_SIZE), Val::Px(SKILL_ICON_SIZE)),
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect {
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    top: Val::Px(20.),
                                    bottom: Val::Px(60.),
                                },
                                ..Default::default()
                            },
                            image: skill_image.r[0].clone().into(),
                            ..Default::default()
                        })
                        .insert(SkillR)
                        .insert(r);

                    fn get_spell_img(spell: Spell, textures: &Res<Textures>) -> Handle<Image> {
                        match spell {
                            Spell::BARRIER => textures.spell.barrier.clone(),
                            Spell::CLARITY => textures.spell.clarity.clone(),
                            Spell::CLEANSE => textures.spell.cleanse.clone(),
                            Spell::EXHAUST => textures.spell.exhaust.clone(),
                            Spell::FLASH => textures.spell.flash.clone(),
                            Spell::GHOST => textures.spell.ghost.clone(),
                            Spell::HEAL => textures.spell.heal.clone(),
                            Spell::IGNITE => textures.spell.ignite.clone(),
                            Spell::MARK => textures.spell.mark.clone(),
                            Spell::SMITE => textures.spell.smite.clone(),
                            Spell::TELEPORT => textures.spell.teleport.clone(),
                        }
                    }

                    let icon_d = get_spell_img(game_info.spell_d, &textures);
                    let icon_f = get_spell_img(game_info.spell_f, &textures);

                    // D
                    commands
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(SPELL_ICON_SIZE), Val::Px(SPELL_ICON_SIZE)),
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect {
                                    left: Val::Px(15.),
                                    right: Val::Px(5.),
                                    top: Val::Px(20.),
                                    bottom: Val::Px(60.),
                                },
                                ..Default::default()
                            },
                            image: icon_d.into(),
                            ..Default::default()
                        })
                        .insert(SpellD);

                    // F
                    commands
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(SPELL_ICON_SIZE), Val::Px(SPELL_ICON_SIZE)),
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect {
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    top: Val::Px(20.),
                                    bottom: Val::Px(60.),
                                },
                                ..Default::default()
                            },
                            image: icon_f.into(),
                            ..Default::default()
                        })
                        .insert(SpellF);
                });
        });
}
