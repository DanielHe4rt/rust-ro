#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::highpriest_base::{*};

impl Skill for Assumptio {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for Assumptio {
}
impl Skill for Basilica {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Basilica {
}
impl Skill for Meditatio {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for Meditatio {
}
impl Skill for ManaRecharge {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for ManaRecharge {
}
