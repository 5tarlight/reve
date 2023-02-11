use bevy::prelude::Resource;

pub const LOGO: &str = "REVE";

#[derive(Resource)]
pub struct GameInfo {
    pub username: String,
    pub token: String,
    pub server: String,
    pub room: String,
}
