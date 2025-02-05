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
// CR_SLIMPITCHER
pub struct AidCondensedPotion {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AidCondensedPotion {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        478
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
       3
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
       30
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 30 { Ok(30) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 545, name_english: "Red_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 545, name_english: "Red_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 545, name_english: "Red_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 545, name_english: "Red_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 545, name_english: "Red_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 546, name_english: "Yellow_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 546, name_english: "Yellow_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 546, name_english: "Yellow_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 546, name_english: "Yellow_Slim_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 547, name_english: "White_Slim_Potion".to_string(), amount: 1})]; 
        if inventory.iter().find(|item| item.item_id == 545 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 545 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 545 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 545 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 545 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 546 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 546 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 546 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 546 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 547 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for AidCondensedPotion {
}
// CR_FULLPROTECTION
pub struct FullProtection {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for FullProtection {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        479
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
       1
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
       40
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Support
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 40 { Ok(40) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 7139, name_english: "Coating_Bottle".to_string(), amount: 1})]; 
        if inventory.iter().find(|item| item.item_id == 7139 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {
        Some(self)
    }
}
impl SupportiveSkillBase for FullProtection {
}
// CR_ACIDDEMONSTRATION
pub struct AcidDemonstration {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for AcidDemonstration {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        490
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
       9
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
       30
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Attack
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 30 { Ok(30) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 7135, name_english: "Fire_Bottle".to_string(), amount: 1}),(NormalInventoryItem {item_id: 7136, name_english: "Acid_Bottle".to_string(), amount: 1})]; 
        if inventory.iter().find(|item| item.item_id == 7135 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 7136 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
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
impl OffensiveSkillBase for AcidDemonstration {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
        if self.level == 1 {
            return 1
        }
        if self.level == 2 {
            return 2
        }
        if self.level == 3 {
            return 3
        }
        if self.level == 4 {
            return 4
        }
        if self.level == 5 {
            return 5
        }
        if self.level == 6 {
            return 6
        }
        if self.level == 7 {
            return 7
        }
        if self.level == 8 {
            return 8
        }
        if self.level == 9 {
            return 9
        }
        if self.level == 10 {
            return 10
        }
        0
    }
}
// CR_CULTIVATION
pub struct PlantCultivation {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for PlantCultivation {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        491
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
       1
    }
    fn _is_ranged(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _max_level(&self) -> u8 {
        2
    }
    #[inline(always)]
    fn _sp_cost(&self) -> u16 {
       10
    }
    fn _target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if *status.sp() > 10 { Ok(10) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 921, name_english: "Mushroom_Spore".to_string(), amount: 1}),(NormalInventoryItem {item_id: 905, name_english: "Stem".to_string(), amount: 1})]; 
        if inventory.iter().find(|item| item.item_id == 921 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 905 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for PlantCultivation {
}
