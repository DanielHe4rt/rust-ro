// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::item::WearWeapon;

use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;

use crate::{*};

use crate::base::*;
use std::any::Any;
// LK_AURABLADE
pub struct AuraBlade {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AuraBlade {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        355
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 18
        }
        if self.level == 2 {
            return 26
        }
        if self.level == 3 {
            return 34
        }
        if self.level == 4 {
            return 42
        }
        if self.level == 5 {
            return 50
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8388606 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for AuraBlade {
}
// LK_PARRYING
pub struct Parrying {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Parrying {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        356
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        10
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       50
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 50 { Ok(50) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for Parrying {
}
// LK_CONCENTRATION
pub struct Concentration {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Concentration {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        357
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 14
        }
        if self.level == 2 {
            return 18
        }
        if self.level == 3 {
            return 22
        }
        if self.level == 4 {
            return 26
        }
        if self.level == 5 {
            return 30
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for Concentration {
}
// LK_TENSIONRELAX
pub struct Relax {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Relax {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        358
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       15
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 15 { Ok(15) } else {Err(())}
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for Relax {
}
// LK_BERSERK
pub struct Frenzy {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Frenzy {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        359
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
        0
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        1
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       200
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 200 { Ok(200) } else {Err(())}
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for Frenzy {
}
// LK_SPIRALPIERCE
pub struct SpiralPierce {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SpiralPierce {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        397
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       5
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 18
        }
        if self.level == 2 {
            return 21
        }
        if self.level == 3 {
            return 24
        }
        if self.level == 4 {
            return 27
        }
        if self.level == 5 {
            return 30
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Attack
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 300
        }
        if self.level == 2 {
            return 500
        }
        if self.level == 3 {
            return 700
        }
        if self.level == 4 {
            return 900
        }
        if self.level == 5 {
            return 1000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 1200
        }
        if self.level == 2 {
            return 1400
        }
        if self.level == 3 {
            return 1600
        }
        if self.level == 4 {
            return 1800
        }
        if self.level == 5 {
            return 2000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 1200
        }
        if self.level == 2 {
            return 1400
        }
        if self.level == 3 {
            return 1600
        }
        if self.level == 4 {
            return 1800
        }
        if self.level == 5 {
            return 2000
        }
        0
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for SpiralPierce {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       5
    }
}
// LK_HEADCRUSH
pub struct TraumaticBlow {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for TraumaticBlow {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        398
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       4
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        5
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       23
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Attack
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 23 { Ok(23) } else {Err(())}
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       500
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for TraumaticBlow {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.400)
        }
        if self.level == 2 {
            return Some(1.800)
        }
        if self.level == 3 {
            return Some(2.200)
        }
        if self.level == 4 {
            return Some(2.600)
        }
        if self.level == 5 {
            return Some(3.000)
        }
        None
    }
}
// LK_JOINTBEAT
pub struct VitalStrike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for VitalStrike {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        399
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _range(&self) -> i8 {
       4
    }
    fn _is_ranged(&self) -> bool {
        true
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        10
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 12
        }
        if self.level == 2 {
            return 12
        }
        if self.level == 3 {
            return 14
        }
        if self.level == 4 {
            return 14
        }
        if self.level == 5 {
            return 16
        }
        if self.level == 6 {
            return 16
        }
        if self.level == 7 {
            return 18
        }
        if self.level == 8 {
            return 18
        }
        if self.level == 9 {
            return 20
        }
        if self.level == 10 {
            return 20
        }
        0
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Attack
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if *status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if *status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 3 {
            if *status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 4 {
            if *status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 5 {
            if *status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 6 {
            if *status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 7 {
            if *status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 8 {
            if *status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 9 {
            if *status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 10 {
            if *status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 48 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 800
        }
        if self.level == 2 {
            return 800
        }
        if self.level == 3 {
            return 800
        }
        if self.level == 4 {
            return 800
        }
        if self.level == 5 {
            return 800
        }
        if self.level == 6 {
            return 1000
        }
        if self.level == 7 {
            return 1000
        }
        if self.level == 8 {
            return 1000
        }
        if self.level == 9 {
            return 1000
        }
        if self.level == 10 {
            return 1000
        }
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 800
        }
        if self.level == 2 {
            return 800
        }
        if self.level == 3 {
            return 800
        }
        if self.level == 4 {
            return 800
        }
        if self.level == 5 {
            return 800
        }
        if self.level == 6 {
            return 1000
        }
        if self.level == 7 {
            return 1000
        }
        if self.level == 8 {
            return 1000
        }
        if self.level == 9 {
            return 1000
        }
        if self.level == 10 {
            return 1000
        }
        0
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for VitalStrike {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(0.600)
        }
        if self.level == 2 {
            return Some(0.700)
        }
        if self.level == 3 {
            return Some(0.800)
        }
        if self.level == 4 {
            return Some(0.900)
        }
        if self.level == 5 {
            return Some(1.000)
        }
        if self.level == 6 {
            return Some(1.100)
        }
        if self.level == 7 {
            return Some(1.200)
        }
        if self.level == 8 {
            return Some(1.300)
        }
        if self.level == 9 {
            return Some(1.400)
        }
        if self.level == 10 {
            return Some(1.500)
        }
        None
    }
}
