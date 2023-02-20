use bevy::prelude::Component;

use crate::constants::Spells;

#[derive(Component)]
pub struct SkillP;

#[derive(Component)]
pub struct SkillQ;

#[derive(Component)]
pub struct SkillW;

#[derive(Component)]
pub struct SkillE;

#[derive(Component)]
pub struct SkillR;

#[derive(Component)]
pub struct SpellD;

#[derive(Component)]
pub struct SpellF;

#[derive(Clone)]
pub enum SkillCooldown {
    /// Normal time based cooldown system
    /// Skill Acceleration will affect this cooldown
    Sec(Vec<f32>),
    /// Conditinally enabled skill
    /// If the value is true skill will be available
    Con(bool),
}

#[derive(Clone)]
pub enum SkillCost {
    Mp(Vec<f32>),
    NoCost,
}

#[derive(Clone, Copy)]
pub enum SkillStatus {
    /// Skill Ready!
    Available,
    /// Skill is running now
    Active,
    /// Not enough mana to use skill
    NoMp,
    /// Skill is under cooldown
    Cooldown,
    /// Skill disabled
    /// This does not mean some conditions required not acquired
    /// This means skill disabled by supression or so on
    Disabled,
    /// This means some requirements not qcquired
    StandBy,
    /// Cannot use this skill because of lack of skill point
    NotHave,
}

#[derive(Component)]
pub struct SkillStat {
    pub cool: SkillCooldown,
    pub status: SkillStatus,
    pub cost: SkillCost,
}

#[derive(Clone, Copy)]
pub enum SpellCooldown {
    Sec(f32),
}

#[derive(Clone, Copy)]
pub enum SpellStatus {
    Available,
    Cooldown,
    Disabled,
}

#[derive(Component)]
pub struct SpellStat {
    pub cooldown: SpellCooldown,
    pub status: SpellStatus,
}

pub struct Spell;

impl Spell {
    pub fn get_spell_stat(spell: Spells) -> SpellStat {
        match spell {
            Spells::SMITE => SpellStat {
                cooldown: SpellCooldown::Sec(90.),
                status: SpellStatus::Available,
            },
            Spells::GHOST => SpellStat {
                cooldown: SpellCooldown::Sec(210.),
                status: SpellStatus::Available,
            },
            Spells::HEAL => SpellStat {
                cooldown: SpellCooldown::Sec(240.),
                status: SpellStatus::Available,
            },
            Spells::TELEPORT => SpellStat {
                cooldown: SpellCooldown::Sec(360.),
                status: SpellStatus::Available,
            },
            Spells::CLEANSE => SpellStat {
                cooldown: SpellCooldown::Sec(210.),
                status: SpellStatus::Available,
            },
            Spells::BARRIER => SpellStat {
                cooldown: SpellCooldown::Sec(180.),
                status: SpellStatus::Available,
            },
            Spells::IGNITE => SpellStat {
                cooldown: SpellCooldown::Sec(180.),
                status: SpellStatus::Available,
            },
            Spells::EXHAUST => SpellStat {
                cooldown: SpellCooldown::Sec(210.),
                status: SpellStatus::Available,
            },
            Spells::FLASH => SpellStat {
                cooldown: SpellCooldown::Sec(300.),
                status: SpellStatus::Available,
            },
            Spells::MARK => SpellStat {
                cooldown: SpellCooldown::Sec(48.),
                status: SpellStatus::Available,
            },
            Spells::CLARITY => SpellStat {
                cooldown: SpellCooldown::Sec(240.),
                status: SpellStatus::Available,
            },
        }
    }
}
