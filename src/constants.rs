use bevy::prelude::{Handle, Image, Resource};

use crate::champion::Champions;

pub const MIN_MAP_X: f32 = -6100.0;
pub const MIN_MAP_Y: f32 = -5120.0;
pub const MAX_MAP_X: f32 = 6100.0;
pub const MAX_MAP_Y: f32 = 5120.0;
pub const CAM_SPEED: f32 = 1500.0;
pub const CAM_GAP: f32 = 25.0;
pub const CAM_KEY_SPEED: f32 = 1200.;

pub const LOGO: &str = "REVE";
pub const MAP: &str = "img/map.png";
pub const CIRCLE: &str = "img/circle.png";
pub const CURSOR_DURATION: f32 = 0.2;

pub const PORTRAIT_SCALE: f32 = 0.8;
pub const SKILL_UI_W: f32 = 400.;
pub const SKILL_UI_H: f32 = 160.;
pub const PASSIVE_ICON_SIZE: f32 = 32.;
pub const SKILL_ICON_SIZE: f32 = 48.;
pub const SPELL_ICON_SIZE: f32 = 40.;

pub const BARRIER: &str = "img/skill/spell/barrier.png";
pub const CLARITY: &str = "img/skill/spell/clarity.png";
pub const CLEANSE: &str = "img/skill/spell/cleanse.png";
pub const EXHAUST: &str = "img/skill/spell/exhaust.png";
pub const FLASH: &str = "img/skill/spell/flash.png";
pub const GHOST: &str = "img/skill/spell/ghost.png";
pub const HEAL: &str = "img/skill/spell/heal.png";
pub const IGNITE: &str = "img/skill/spell/ignite.png";
pub const MARK: &str = "img/skill/spell/mark.png";
pub const SMITE: &str = "img/skill/spell/smite.png";
pub const TELEPORT: &str = "img/skill/spell/teleport.png";

pub const GAREN: &str = "img/champ/Garen.png";
pub const GAREN_P: &str = "img/skill/garen/p.png";
pub const GAREN_Q: &str = "img/skill/garen/q.png";
pub const GAREN_W: &str = "img/skill/garen/w.png";
pub const GAREN_E: &str = "img/skill/garen/e.png";
pub const GAREN_E_CANCEL: &str = "img/skill/garen/e_cancel.png";
pub const GAREN_R: &str = "img/skill/garen/r.png";

pub const ASH: &str = "img/champ/Ash.png";
pub const ASH_P: &str = "img/skill/ash/p.png";
pub const ASH_Q: &str = "img/skill/ash/q.png";
pub const ASH_W: &str = "img/skill/ash/w.png";
pub const ASH_E: &str = "img/skill/ash/e.png";
pub const ASH_R: &str = "img/skill/ash/r.png";

#[derive(Resource)]
pub struct Textures {
    pub map: Handle<Image>,
    pub cursor: Handle<Image>,
    pub spell: SpellTexture,
    pub garen: ChampionTexture,
    pub ash: ChampionTexture,
}

#[derive(Resource)]
pub struct SpellTexture {
    pub barrier: Handle<Image>,
    pub clarity: Handle<Image>,
    pub cleanse: Handle<Image>,
    pub exhaust: Handle<Image>,
    pub flash: Handle<Image>,
    pub ghost: Handle<Image>,
    pub heal: Handle<Image>,
    pub ignite: Handle<Image>,
    pub mark: Handle<Image>,
    pub smite: Handle<Image>,
    pub teleport: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct ChampionTexture {
    pub portrait: Handle<Image>,
    pub p: Vec<Handle<Image>>,
    pub q: Vec<Handle<Image>>,
    pub w: Vec<Handle<Image>>,
    pub e: Vec<Handle<Image>>,
    pub r: Vec<Handle<Image>>,
}

#[derive(Clone, Copy)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Resource, Clone, Copy)]
pub enum Spell {
    BARRIER,
    CLARITY,
    CLEANSE,
    EXHAUST,
    FLASH,
    GHOST,
    HEAL,
    IGNITE,
    MARK,
    SMITE,
    TELEPORT,
}

#[derive(Resource, Clone)]
pub struct GameInfo {
    pub username: String,
    pub token: String,
    pub server: String,
    pub room: String,
    pub team: Team,
    pub champion: Champions,
    pub spell_d: Spell,
    pub spell_f: Spell,
}
