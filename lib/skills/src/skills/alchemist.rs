#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::alchemist_base::{*};

impl Skill for AxeMastery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for AxeMastery {
}
impl Skill for PotionResearch {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for PotionResearch {
}
impl Skill for PreparePotion {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for PreparePotion {
}
impl Skill for Bomb {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for Bomb {
}
impl GroundSkill for Bomb {
}
impl Skill for AcidTerror {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for AcidTerror {
}
impl Skill for AidPotion {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for AidPotion {
}
impl Skill for SummonFlora {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl GroundSkill for SummonFlora {
}
impl Skill for SummonMarineSphere {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl GroundSkill for SummonMarineSphere {
}
impl Skill for AlchemicalWeapon {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for AlchemicalWeapon {
}
impl Skill for SynthesizedShield {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for SynthesizedShield {
}
impl Skill for SyntheticArmor {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for SyntheticArmor {
}
impl Skill for BiochemicalHelm {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for BiochemicalHelm {
}
impl Skill for Bioethics {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for Bioethics {
}
impl Skill for CallHomunculus {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for CallHomunculus {
}
impl Skill for Vaporize {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Vaporize {
}
impl Skill for HomunculusResurrection {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for HomunculusResurrection {
}
impl Skill for AidBerserkPotion {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for AidBerserkPotion {
}
impl Skill for TwilightAlchemy1 {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for TwilightAlchemy1 {
}
impl Skill for TwilightAlchemy2 {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for TwilightAlchemy2 {
}
impl Skill for TwilightAlchemy3 {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for TwilightAlchemy3 {
}
