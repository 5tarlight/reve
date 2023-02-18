use bevy::prelude::{Handle, Image, Resource};

use crate::champion::Champions;

pub const MIN_MAP_X: f32 = -6100.0;
pub const MIN_MAP_Y: f32 = -5120.0;
pub const MAX_MAP_X: f32 = 6100.0;
pub const MAX_MAP_Y: f32 = 5120.0;
pub const CAM_SPEED: f32 = 1500.0;
pub const CAM_GAP: f32 = 100.0;
pub const CAM_KEY_SPEED: f32 = 1200.;

pub const LOGO: &str = "REVE";
pub const MAP: &str = "img/map.png";
pub const CIRCLE: &str = "img/circle.png";
pub const CURSOR_DURATION: f32 = 0.2;

pub const GAREN: &str = "img/champ/Garen.png";
pub const GAREN_P: &str = "img/skill/garen/p.png";
pub const GAREN_Q: &str = "img/skill/garen/q.png";
pub const GAREN_W: &str = "img/skill/garen/w.png";
pub const GAREN_E: &str = "img/skill/garen/e.png";
pub const GAREN_E_CANCEL: &str = "img/skill/garen/p_cancel.png";
pub const GAREN_R: &str = "img/skill/garen/r.png";

#[derive(Resource)]
pub struct Textures {
    pub map: Handle<Image>,
    pub cursor: Handle<Image>,
    pub garen: GarenTexture,
}

#[derive(Resource, Clone)]
pub struct GarenTexture {
    pub portrait: Handle<Image>,
    pub p: Handle<Image>,
    pub q: Handle<Image>,
    pub w: Handle<Image>,
    pub e: Handle<Image>,
    pub e_cancel: Handle<Image>,
    pub r: Handle<Image>,
}

#[derive(Clone, Copy)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Resource)]
pub struct GameInfo {
    pub username: String,
    pub token: String,
    pub server: String,
    pub room: String,
    pub team: Team,
    pub champion: Champions,
}
