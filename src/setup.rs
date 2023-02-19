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
        ChampionTexture, GameInfo, Spell, SpellTexture, Team, Textures, ASH, ASH_E, ASH_P, ASH_Q,
        ASH_R, ASH_W, BARRIER, CIRCLE, CLARITY, CLEANSE, EXHAUST, FLASH, GAREN, GAREN_E,
        GAREN_E_CANCEL, GAREN_P, GAREN_Q, GAREN_R, GAREN_W, GHOST, HEAL, IGNITE, MAP, MARK, SMITE,
        TELEPORT,
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

    let champion = match champion.as_str() {
        "Garen" => Champions::GAREN,
        "Ash" => Champions::ASH,
        _ => Champions::GAREN,
    };

    fn parse_spell(spell: &String) -> Spell {
        match spell.as_str() {
            "barrier" => Spell::BARRIER,
            "clarity" => Spell::CLARITY,
            "cleanse" => Spell::CLEANSE,
            "exhaust" => Spell::EXHAUST,
            "flash" => Spell::FLASH,
            "ghost" => Spell::GHOST,
            "heal" => Spell::HEAL,
            "ignite" => Spell::IGNITE,
            "mark" => Spell::MARK,
            "smite" => Spell::SMITE,
            "teleport" => Spell::TELEPORT,
            _ => Spell::FLASH,
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

    match game_info.champion {
        Champions::GAREN => {
            champ = Champion {
                max_hp: 690.,
                hp: 690.,
                grow_hp: 98.,
                hp_gen: 8.,
                grow_hp_gen: 0.5,
                ad: 66.,
                grow_ad: 4.5,
                ap: 0.,
                grow_ap: 0.,
                attack_speed: 0.625,
                grow_attack_speed: 3.65,
                def_ad: 36.,
                grow_def_ad: 4.2,
                def_ap: 32.,
                grow_def_ap: 1.55,
                hit_range: 175.,
                move_speed: 340.,
                crit_prob: 0,
                cooldown: 0.,
                name: Champions::GAREN,
                team: game_info.team.clone(),
            };
            img = textures.garen.portrait.clone();
        }
    }

    let ms = champ.move_speed;
    commands
        .spawn(SpriteBundle {
            texture: img,
            transform: Transform {
                scale: Vec3::splat(0.3),
                translation: Vec3::new(-4800.0, -4400., 2.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(champ)
        .insert(MyPlayer)
        .insert(Velocity(ms));
}
