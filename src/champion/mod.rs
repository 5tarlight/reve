use bevy::prelude::Component;

use crate::{
    constants::Team,
    skill::{SkillCooldown, SkillCost, SkillStat, SkillStatus},
};

pub mod class;

#[derive(Clone, Copy)]
pub enum Champions {
    GAREN,
    ASH,
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
    pub mp: f32,
    pub max_mp: f32,
    pub grow_mp: f32,
    pub mp_gen: f32,
    pub grow_mp_gen: f32,
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

impl Champion {
    pub fn new(ch: Champions, team: Team) -> Self {
        match ch {
            Champions::GAREN => Self {
                max_hp: 690.,
                hp: 690.,
                grow_hp: 98.,
                hp_gen: 8.,
                grow_hp_gen: 0.5,
                mp: 0.,
                grow_mp: 0.,
                grow_mp_gen: 0.,
                max_mp: 0.,
                mp_gen: 0.,
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
                team: team,
            },
            Champions::ASH => Self {
                hp: 640.,
                max_hp: 640.,
                grow_hp: 101.,
                hp_gen: 3.5,
                grow_hp_gen: 0.55,
                mp: 280.,
                max_mp: 280.,
                grow_mp: 35.,
                mp_gen: 6.97,
                grow_mp_gen: 0.65,
                ad: 59.,
                grow_ad: 2.96,
                ap: 0.,
                grow_ap: 0.,
                attack_speed: 0.658,
                grow_attack_speed: 3.33,
                def_ad: 26.,
                grow_def_ad: 4.6,
                def_ap: 30.,
                grow_def_ap: 1.3,
                hit_range: 600.,
                move_speed: 325.,
                cooldown: 0.,
                crit_prob: 0,
                name: Champions::ASH,
                team,
            },
        }
    }

    pub fn get_skill_stats(ch: Champions) -> [SkillStat; 4] {
        match ch {
            Champions::GAREN => {
                let q = SkillStat {
                    cool: SkillCooldown::Sec(vec![8., 8., 8., 8., 8.]),
                    status: SkillStatus::Available,
                    cost: SkillCost::NoCost,
                };
                let w = SkillStat {
                    cool: SkillCooldown::Sec(vec![23., 21., 19., 17., 15.]),
                    status: SkillStatus::NotHave,
                    cost: SkillCost::NoCost,
                };
                let e = SkillStat {
                    cool: SkillCooldown::Sec(vec![9., 9., 9., 9., 9.]),
                    status: SkillStatus::NotHave,
                    cost: SkillCost::NoCost,
                };
                let r = SkillStat {
                    cool: SkillCooldown::Sec(vec![120., 100., 80.]),
                    status: SkillStatus::NotHave,
                    cost: SkillCost::NoCost,
                };

                [q, w, e, r]
            }
            Champions::ASH => {
                let q = SkillStat {
                    cool: SkillCooldown::Con(false),
                    status: SkillStatus::NotHave,
                    cost: SkillCost::Mp(vec![50., 50., 50., 50., 50.]),
                };
                let w = SkillStat {
                    cool: SkillCooldown::Sec(vec![23., 21., 19., 17., 15.]),
                    status: SkillStatus::NotHave,
                    cost: SkillCost::Mp(vec![75., 70., 65., 60., 55.]),
                };
                let e = SkillStat {
                    cool: SkillCooldown::Sec(vec![90., 80., 70., 60., 50.]),
                    status: SkillStatus::NotHave,
                    cost: SkillCost::NoCost,
                };
                let r = SkillStat {
                    cool: SkillCooldown::Sec(vec![80., 70., 60.]),
                    status: SkillStatus::NotHave,
                    cost: SkillCost::Mp(vec![100., 100., 100., 100., 100.]),
                };

                [q, w, e, r]
            }
        }
    }
}

#[derive(Component)]
pub struct MyPlayer;
