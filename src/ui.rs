use bevy::{
    prelude::{
        BuildChildren, ButtonBundle, Commands, Component, NodeBundle, Plugin, Res, StartupStage,
    },
    ui::{AlignSelf, JustifyContent, Size, Style, UiRect, Val},
};

use crate::{
    champion::Champions,
    constants::{GameInfo, Textures, PASSIVE_ICON_SIZE, SKILL_ICON_SIZE},
    skill::{SkillE, SkillP, SkillQ, SkillR, SkillW},
};

pub struct ReveUiPlugin;

impl Plugin for ReveUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init_ui);
    }
}

#[derive(Component)]
pub struct SkillUiParent;

fn init_ui(mut commands: Commands, textures: Res<Textures>, game_info: Res<GameInfo>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SkillUiParent)
        .with_children(|commands| {
            // TODO : update skill images that should be changed

            let skill_image = match game_info.champion {
                Champions::Garen => &textures.garen,
            };

            // Passive
            commands
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(PASSIVE_ICON_SIZE), Val::Px(PASSIVE_ICON_SIZE)),
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
                .insert(SkillQ);

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
                .insert(SkillW);

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
                .insert(SkillE);

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
                .insert(SkillR);
        });
}
