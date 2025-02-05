use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::mob_model::MobModel;
use configuration::configuration::GameConfig;
use enums::size::Size;
use enums::EnumWithStringValue;
use models::status::{KnownSkill, Look, Status};

pub struct StatusFromDb;
impl StatusFromDb {
    pub fn from_char_model(char_model: &CharSelectModel, configuration: &GameConfig, known_skills: Vec<KnownSkill>) -> Status {
        Status {
            job: char_model.class as u32,
            hp: char_model.hp as u32,
            sp: char_model.sp as u32,
            max_hp: char_model.max_hp as u32,
            max_sp: char_model.max_sp as u32,
            str: char_model.str as u16,
            agi: char_model.agi as u16,
            vit: char_model.vit as u16,
            int: char_model.int as u16,
            dex: char_model.dex as u16,
            luk: char_model.luk as u16,
            base_atk: 0,
            matk_min: 0,
            matk_max: 0,
            speed: configuration.default_char_speed,
            hit: 0,
            flee: 0,
            crit: 0,
            def: 0,
            mdef: 0,
            look: Some(Look {
                hair: char_model.hair as u16,
                hair_color: char_model.hair_color as u32,
                clothes_color: char_model.clothes_color as u32,
                body: char_model.body as u32,
                weapon: char_model.weapon as u32,
                shield: char_model.shield as u32,
                head_top: char_model.head_top as u32,
                head_middle: char_model.head_mid as u32,
                head_bottom: char_model.head_bottom as u32,
                robe: char_model.robe as u32,
            }),
            zeny: char_model.zeny as u32,
            base_level: char_model.base_level as u32,
            job_level: char_model.job_level as u32,
            status_point: char_model.status_point as u32,
            skill_point: char_model.skill_point as u32,
            base_exp: char_model.base_exp as u32,
            job_exp: char_model.job_exp as u32,
            state: 0,
            size: Default::default(),
            weapons: vec![],
            equipments: vec![],
            ammo: None,
            known_skills,
        }
    }
    pub fn from_mob_model(mob_model: &MobModel) -> Status {
        Status {
            job: mob_model.id as u32,
            hp: mob_model.hp as u32,
            sp: mob_model.sp as u32,
            max_hp: mob_model.hp as u32,
            max_sp: mob_model.sp as u32,
            str: mob_model.str as u16,
            agi: mob_model.agi as u16,
            vit: mob_model.vit as u16,
            int: mob_model.int as u16,
            dex: mob_model.dex as u16,
            luk: mob_model.luk as u16,
            base_atk: mob_model.atk1 as u32,
            matk_min: mob_model.atk1 as u32,
            matk_max: mob_model.atk2 as u32,
            speed: mob_model.speed as u16,
            hit: 0,
            flee: 0,
            crit: 0,
            def: mob_model.def as u32,
            mdef: mob_model.mdef as u32,
            look: None,
            zeny: 0,
            base_level: 0,
            job_level: 0,
            status_point: 0,
            skill_point: 0,
            base_exp: 0,
            job_exp: 0,
            state: 0,
            size: Size::from_string(&mob_model.size),
            weapons: vec![],
            equipments: vec![],
            ammo: None,
            known_skills: vec![],
        }
    }
}