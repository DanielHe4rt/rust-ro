#![allow(dead_code)]
use crate::*;

#[derive(Debug, WithNumberValue)]
pub enum StatusTypes {
    #[value = 0]
    Speed,
    Baseexp,
    Jobexp,
    Karma,
    Manner,
    Hp,
    Maxhp,
    Sp,
    Maxsp,
    Statuspoint,
    #[value = 11]
    Baselevel,
    Skillpoint,
    Str,
    Agi,
    Vit,
    Int,
    Dex,
    Luk,
    Class,
    Zeny,
    Sex,
    Nextbaseexp,
    Nextjobexp,
    Weight,
    Maxweight,
    #[value = 32]
    StrNextLevelIncreaseCost,
    AgiNextLevelIncreaseCost,
    VitNextLevelIncreaseCost,
    IntNextLevelIncreaseCost,
    DexNextLevelIncreaseCost,
    LukNextLevelIncreaseCost,
    #[value = 41]
    Atk1,
    Atk2,
    Matk1,
    Matk2,
    Def1,
    Def2,
    Mdef1,
    Mdef2,
    Hit,
    Flee1,
    Flee2,
    Critical,
    Aspd,
    #[value = 55]
    Joblevel,
    Upper,
    Partner,
    Cart,
    Fame,
    Unbreakable,
    #[value = 99]
    Cartinfo,
    #[value = 119]
    Basejob,
    Baseclass,
    Killerrid,
    Killedrid,
    Slotchange,
    Charrename,
    Modexp,
    Moddrop,
    Moddeath,
    Bankvault,
    #[value = 165]
    Mercflee,
    Merckills,
    Mercfaith,
    #[value = 219]
    Pow,
    Sta,
    Wis,
    Spl,
    Con,
    Crt,
    Patk,
    Smatk,
    Res,
    Mres,
    Hplus,
    Crate,
    Tstatuspoint,
    Ap,
    Maxap,
    Upow,
    Usta,
    Uwis,
    Uspl,
    Ucon,
    Ucrt,
    #[value = 1000]
    Attackrange,
    Atkele,
    Defele,
    Castrate,
    Maxhprate,
    Maxsprate,
    Sprate,
    Addele,
    Addrace,
    Addsize,
    Subele,
    Subrace,
    Addeff,
    Reseff,
    Baseatk,
    Aspdrate,
    Hprecovrate,
    Recovrate,
    Speedrate,
    Criticaldef,
    Nearatkdef,
    Longatkdef,
    Doublerate,
    Doubleaddrate,
    Skillheal,
    Matkrate,
    Ignoredefele,
    Ignoredefrace,
    Atkrate,
    Speedaddrate,
    Regenrate,
    Magicatkdef,
    Miscatkdef,
    Ignoremdefele,
    Ignoremdefrace,
    Magicaddele,
    Magicaddrace,
    Magicaddsize,
    Perfecthitrate,
    Perfecthitaddrate,
    Criticalrate,
    Getzenynum,
    Addgetzenynum,
    Adddamageclass,
    Addmagicdamageclass,
    Adddefclass,
    Addmdefclass,
    Addmonsterdropitem,
    Defratioatkele,
    Defratioatkrace,
    Unbreakablegarment,
    Hitrate,
    Fleerate,
    Flee2Rate,
    Defrate,
    Def2Rate,
    Mdefrate,
    Mdef2Rate,
    Splashrange,
    Splashaddrange,
    Autospell,
    Hpdrainrate,
    Drainrate,
    Shortweapondamagereturn,
    Longweapondamagereturn,
    Weaponcomaele,
    Weaponcomarace,
    Addeff2,
    Breakweaponrate,
    Breakarmorrate,
    Addstealrate,
    Magicdamagereturn,
    Allstats,
    Agivit,
    Agidexstr,
    Perfecthide,
    Noknockback,
    Classchange,
    Hpdrainvalue,
    Drainvalue,
    Weaponatk,
    Weaponatkrate,
    Delayrate,
    Hpdrainraterace,
    Drainraterace,
    Ignoremdefrate,
    Ignoredefrate,
    Skillheal2,
    Addeffonskill,
    Addhealrate,
    Addheal2Rate,
    Hpvanishrate,
    #[value = 2000]
    Restartfullrecover,
    Nocastcancel,
    Nosizefix,
    Nomagicdamage,
    Noweapondamage,
    Nogemstone,
    Nocastcancel2,
    Nomiscdamage,
    Unbreakableweapon,
    Unbreakablearmor,
    Unbreakablehelm,
    Unbreakableshield,
    Longatkrate,
    Critatkrate,
    Criticaladdrace,
    Noregen,
    Addeffwhenhit,
    Autospellwhenhit,
    Skillatk,
    Unstripable,
    Autospellonskill,
    Gainvalue,
    Hpregenrate,
    Hplossrate,
    Addrace2,
    Hpgainvalue,
    Subsize,
    Hpdrainvaluerace,
    Additemhealrate,
    Drainvaluerace,
    Expaddrace,
    Gainrace,
    Subrace2,
    Unbreakableshoes,
    Unstripableweapon,
    Unstripablearmor,
    Unstripablehelm,
    Unstripableshield,
    Intravision,
    Addmonsterdropchainitem,
    Lossrate,
    Addskillblow,
    Vanishrate,
    Magicgainvalue,
    Magichpgainvalue,
    Addclassdropitem,
    Ematk,
    Gainraceattack,
    Hpgainraceattack,
    Skilluserate,
    Skillcooldown,
    Skillfixedcast,
    Skillvariablecast,
    Fixcastrate,
    Varcastrate,
    Skillusesp,
    Magicatkele,
    Addfixedcast,
    Addvariablecast,
    Setdefrace,
    Setmdefrace,
    Racetolerance,
    Addmaxweight,
    Subdefele,
    Magicsubdefele,
    Statenorecoverrace,
}

impl StatusTypes {
    pub fn to_column(&self) -> Option<&str> {
        match self {
            StatusTypes::Str => Some("str"),
            StatusTypes::Agi => Some("agi"),
            StatusTypes::Vit => Some("vit"),
            StatusTypes::Int => Some("int"),
            StatusTypes::Dex => Some("dex"),
            StatusTypes::Luk => Some("luk"),
            StatusTypes::Baselevel => Some("base_level"),
            StatusTypes::Joblevel => Some("job_level"),
            StatusTypes::Baseexp => Some("base_exp"),
            StatusTypes::Jobexp => Some("job_exp"),
            StatusTypes::Statuspoint => Some("status_point"),
            StatusTypes::Skillpoint => Some("skill_point"),
            _ => None,
        }
    }
}
