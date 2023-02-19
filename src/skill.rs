use bevy::prelude::Component;

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

#[derive(Component)]
pub struct SpellStat {}
