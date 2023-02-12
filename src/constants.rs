use bevy::prelude::{Handle, Image, Resource};

pub const LOGO: &str = "REVE";
pub const MAP: &str = "img/map.png";
pub const GAREN: &str = "img/champ/Garen.png";

#[derive(Resource)]
pub struct Textures {
    pub map: Handle<Image>,
    pub garen: Handle<Image>,
}

pub enum Team {
    Red,
    Blue,
}

pub enum Champion {
    Garen,
}

#[derive(Resource)]
pub struct GameInfo {
    pub username: String,
    pub token: String,
    pub server: String,
    pub room: String,
    pub team: Team,
    pub champion: Champion,
}
