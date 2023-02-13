use bevy::prelude::{Handle, Image, Resource};

use crate::champion::Champions;

pub const MIN_MAP_X: f32 = -5140.0;
pub const MIN_MAP_Y: f32 = -4580.0;
pub const MAX_MAP_X: f32 = 5140.0;
pub const MAX_MAP_Y: f32 = 4580.0;

pub const LOGO: &str = "REVE";
pub const MAP: &str = "img/map.png";
pub const GAREN: &str = "img/champ/Garen.png";

#[derive(Resource)]
pub struct Textures {
    pub map: Handle<Image>,
    pub garen: Handle<Image>,
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
