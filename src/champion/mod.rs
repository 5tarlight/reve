use bevy::prelude::Component;

use crate::constants::Team;

pub mod class;

pub enum Champions {
    Garen,
}

#[derive(Component)]
pub struct Champion {
    pub team: Team,
    pub name: Champions,
    /// Current Health Point
    pub hp: f32,
    /// Max Health Point
    pub max_hp: f32,
    pub grow_hp: f32,
    pub hp_gen: f32,
    pub grow_hp_gen: f32,
    /// Attack damage
    pub ad: f32,
    pub grow_ad: f32,
    /// Ability Damage
    pub ap: f32,
    pub grow_ap: f32,
    pub hit_range: f32,
    /// Physical Endurance
    pub def_ad: f32,
    pub grow_def_ad: f32,
    /// Magical Endurance
    pub def_ap: f32,
    pub grow_def_ap: f32,
    pub attack_speed: f32,
    /// This value is not raw. It is percentage.
    pub grow_attack_speed: f32,
    /// Skill cooldown reduce
    pub cooldown: f32,
    /// Critical Probability
    pub crit_prob: u8,
    /// Movement Speed
    pub move_speed: f32,
}

#[derive(Component)]
pub struct MyPlayer;
