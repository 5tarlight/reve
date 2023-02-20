use bevy::{
    prelude::{
        AssetServer, Camera2d, Camera2dBundle, Commands, Handle, Image, Plugin, Query, Res,
        StartupStage, Transform, Vec3, With,
    },
    sprite::SpriteBundle,
};

use crate::{
    champion::{Champion, Champions, MyPlayer},
    constants::{
        ChampionTexture, GameInfo, SpellTexture, Spells, Team, Textures, ASH, ASH_E, ASH_P, ASH_Q,
        ASH_R, ASH_W, BARRIER, CIRCLE, CLARITY, CLEANSE, EXHAUST, FLASH, GAREN, GAREN_E,
        GAREN_E_CANCEL, GAREN_P, GAREN_Q, GAREN_R, GAREN_W, GHOST, HEAL, IGNITE, MAP, MARK,
        PORTRAIT_SCALE, RIX_FONT, SMITE, TELEPORT,
    },
    movement::Velocity,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup)
            .add_startup_system_to_stage(StartupStage::PostStartup, init_game);
    }
}

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load args
    let args = std::env::args().collect::<Vec<String>>();
    dbg!(&args);

    let username = args[1].clone();
    let token = args[2].clone();
    let server = args[3].clone();
    let room = args[4].clone();
    let team = args[5].clone();
    let champion = args[6].clone();
    let spell_d = args[7].clone();
    let spell_f = args[8].clone();

    let champion = match champion.to_lowercase().as_str() {
        "garen" => Champions::GAREN,
        "ash" => Champions::ASH,
        _ => panic!("Unknown champion: {}", champion),
    };

    fn parse_spell(spell: &String) -> Spells {
        match spell.as_str() {
            "barrier" => Spells::BARRIER,
            "clarity" => Spells::CLARITY,
            "cleanse" => Spells::CLEANSE,
            "exhaust" => Spells::EXHAUST,
            "flash" => Spells::FLASH,
            "ghost" => Spells::GHOST,
            "heal" => Spells::HEAL,
            "ignite" => Spells::IGNITE,
            "mark" => Spells::MARK,
            "smite" => Spells::SMITE,
            "teleport" => Spells::TELEPORT,
            _ => panic!("Unknown Spell: {}", spell),
        }
    }

    let spell_d = parse_spell(&spell_d);
    let spell_f = parse_spell(&spell_f);

    let game_info = GameInfo {
        username,
        token,
        server,
        room,
        team: team.eq("red").then(|| Team::Red).unwrap_or(Team::Blue),
        champion,
        spell_d,
        spell_f,
    };

    commands.insert_resource(game_info);

    // Load assets
    let textures = Textures {
        map: asset.load(MAP),
        cursor: asset.load(CIRCLE),
        rix_font: asset.load(RIX_FONT),
        spell: SpellTexture {
            barrier: asset.load(BARRIER),
            clarity: asset.load(CLARITY),
            cleanse: asset.load(CLEANSE),
            exhaust: asset.load(EXHAUST),
            flash: asset.load(FLASH),
            ghost: asset.load(GHOST),
            heal: asset.load(HEAL),
            ignite: asset.load(IGNITE),
            mark: asset.load(MARK),
            smite: asset.load(SMITE),
            teleport: asset.load(TELEPORT),
        },
        garen: ChampionTexture {
            portrait: asset.load(GAREN),
            p: vec![asset.load(GAREN_P)],
            q: vec![asset.load(GAREN_Q)],
            w: vec![asset.load(GAREN_W)],
            e: vec![asset.load(GAREN_E), asset.load(GAREN_E_CANCEL)],
            r: vec![asset.load(GAREN_R)],
        },
        ash: ChampionTexture {
            portrait: asset.load(ASH),
            p: vec![asset.load(ASH_P)],
            q: vec![asset.load(ASH_Q)],
            w: vec![asset.load(ASH_W)],
            e: vec![asset.load(ASH_E)],
            r: vec![asset.load(ASH_R)],
        },
    };
    commands.insert_resource(textures);
}

fn init_game(
    mut commands: Commands,
    textures: Res<Textures>,
    mut cam_query: Query<&mut Transform, With<Camera2d>>,
    game_info: Res<GameInfo>,
) {
    // Spawn Map
    commands.spawn(SpriteBundle {
        texture: textures.map.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::splat(1.), // TODO: Camera zoom
            ..Default::default()
        },
        ..Default::default()
    });

    // Move camera location depends on team
    let mut cam = cam_query.get_single_mut().unwrap();
    match game_info.team {
        Team::Blue => {
            cam.translation.x = -4800.0;
            cam.translation.y = -4400.0;
        }
        Team::Red => {
            cam.translation.x = 5000.0;
            cam.translation.y = 3800.0;
        }
    }

    // Spawn Player champion
    let champ: Champion;
    let img: Handle<Image>;

    champ = Champion::new(game_info.champion, game_info.team);
    match game_info.champion {
        Champions::GAREN => img = textures.garen.portrait.clone(),
        Champions::ASH => img = textures.ash.portrait.clone(),
    }

    let ms = champ.move_speed;
    commands
        .spawn(SpriteBundle {
            texture: img,
            transform: Transform {
                scale: Vec3::splat(PORTRAIT_SCALE),
                translation: Vec3::new(-4800.0, -4400., 2.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(champ)
        .insert(MyPlayer)
        .insert(Velocity(ms));
}
