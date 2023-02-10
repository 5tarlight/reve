use bevy::prelude::*;
use constants::LOGO;

pub mod constants;

#[derive(Resource)]
#[allow(dead_code)]
pub struct GameInfo {
    username: String,
    token: String,
    server: String,
    room: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "REVE Online".to_string(),
                width: 1920.0,
                height: 1080.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn Logo
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            LOGO,
                            TextStyle {
                                font_size: 200.,
                                color: Color::WHITE,
                                font: asset.load("fonts/d2coding.ttf"),
                            },
                        )
                        .with_text_alignment(TextAlignment::TOP_CENTER),
                    );
                });
        });

    // Load args
    let args = std::env::args().collect::<Vec<String>>();
    dbg!(&args);

    let username = args[1].clone();
    let token = args[2].clone();
    let server = args[3].clone();
    let room = args[4].clone();

    let game_info = GameInfo {
        username,
        token,
        server,
        room,
    };

    commands.insert_resource(game_info);
}
