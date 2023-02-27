use bevy::prelude::{App, Component, Plugin, Query};

use crate::{champion::Champion, ui::PercentWidth};

pub mod minion;

pub struct DamageSystem;

impl Plugin for DamageSystem {
    fn build(&self, app: &mut App) {
        app.add_system(update_champion_stat)
            .add_system(update_champion_stat)
            .add_system(update_percentage);
    }
}

#[derive(Component)]
pub struct Damagable {
    pub hp: f32,
    pub max_hp: f32,
    pub ad_def: f32,
    pub ap_def: f32,
}

impl Default for Damagable {
    fn default() -> Self {
        Self {
            ad_def: 0.,
            ap_def: 0.,
            hp: 0.,
            max_hp: 0.,
        }
    }
}

#[derive(Component)]
pub struct EntityId(pub u32);

fn update_champion_stat(mut query: Query<(&Champion, &mut Damagable)>) {
    for (ch, mut dmg) in query.iter_mut() {
        dmg.max_hp = ch.max_hp;
        dmg.hp = ch.hp;
        dmg.ad_def = ch.def_ad;
        dmg.ap_def = ch.def_ap;
    }
}

fn update_percentage(dmg_query: Query<&Damagable>, mut w_query: Query<&mut PercentWidth>) {}
