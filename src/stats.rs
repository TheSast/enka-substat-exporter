pub(crate) mod rolls;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameStat {
    #[serde(rename = "FIGHT_PROP_ATTACK")]
    Atk,
    #[serde(rename = "FIGHT_PROP_ATTACK_PERCENT")]
    AtkPercentage,
    #[serde(rename = "FIGHT_PROP_DEFENSE")]
    Def,
    #[serde(rename = "FIGHT_PROP_DEFENSE_PERCENT")]
    DefPercentage,
    #[serde(rename = "FIGHT_PROP_HP")]
    Hp,
    #[serde(rename = "FIGHT_PROP_HP_PERCENT")]
    HpPercentage,
    #[serde(rename = "FIGHT_PROP_ELEMENT_MASTERY")]
    ElementalMastery,
    #[serde(rename = "FIGHT_PROP_CHARGE_EFFICIENCY")]
    EnergyRecharge,
    #[serde(rename = "FIGHT_PROP_CRITICAL")]
    CritRate,
    #[serde(rename = "FIGHT_PROP_CRITICAL_HURT")]
    CritDamage,
    #[serde(rename = "FIGHT_PROP_PHYSICAL_ADD_HURT")]
    PhysicalBonus,
    #[serde(rename = "FIGHT_PROP_ROCK_ADD_HURT")]
    GeoBonus,
    #[serde(rename = "FIGHT_PROP_ELEC_ADD_HURT")]
    ElectroBonus,
    #[serde(rename = "FIGHT_PROP_WIND_ADD_HURT")]
    AnemoBonus,
    #[serde(rename = "FIGHT_PROP_GRASS_ADD_HURT")]
    DendroBonus,
    #[serde(rename = "FIGHT_PROP_ICE_ADD_HURT")]
    CryoBonus,
    #[serde(rename = "FIGHT_PROP_WATER_ADD_HURT")]
    HydroBonus,
    #[serde(rename = "FIGHT_PROP_FIRE_ADD_HURT")]
    PyroBonus,
    #[serde(rename = "FIGHT_PROP_HEAL_ADD")]
    HealingBonus,
}

use std::str::FromStr;

impl FromStr for GameStat {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "FIGHT_PROP_ATTACK" => Ok(Self::Atk),
            "FIGHT_PROP_ATTACK_PERCENT" => Ok(Self::AtkPercentage),
            "FIGHT_PROP_DEFENSE" => Ok(Self::Def),
            "FIGHT_PROP_DEFENSE_PERCENT" => Ok(Self::DefPercentage),
            "FIGHT_PROP_HP" => Ok(Self::Hp),
            "FIGHT_PROP_HP_PERCENT" => Ok(Self::HpPercentage),
            "FIGHT_PROP_ELEMENT_MASTERY" => Ok(Self::ElementalMastery),
            "FIGHT_PROP_CHARGE_EFFICIENCY" => Ok(Self::EnergyRecharge),
            "FIGHT_PROP_CRITICAL" => Ok(Self::CritRate),
            "FIGHT_PROP_CRITICAL_HURT" => Ok(Self::CritDamage),
            "FIGHT_PROP_PHYSICAL_ADD_HURT" => Ok(Self::PhysicalBonus),
            "FIGHT_PROP_ROCK_ADD_HURT" => Ok(Self::GeoBonus),
            "FIGHT_PROP_ELEC_ADD_HURT" => Ok(Self::ElectroBonus),
            "FIGHT_PROP_WIND_ADD_HURT" => Ok(Self::AnemoBonus),
            "FIGHT_PROP_GRASS_ADD_HURT" => Ok(Self::DendroBonus),
            "FIGHT_PROP_ICE_ADD_HURT" => Ok(Self::CryoBonus),
            "FIGHT_PROP_WATER_ADD_HURT" => Ok(Self::HydroBonus),
            "FIGHT_PROP_FIRE_ADD_HURT" => Ok(Self::PyroBonus),
            "FIGHT_PROP_HEAL_ADD" => Ok(Self::HealingBonus),
            // Add more as needed
            _ => Err("Unknown stat type"),
        }
    }
}

impl From<GameStat> for f64 {
    fn from(value: GameStat) -> Self {
        match value {
            GameStat::Atk => 19.45,
            GameStat::AtkPercentage => 5.83,
            GameStat::Def => 23.15,
            GameStat::DefPercentage => 7.29,
            GameStat::Hp => 298.75,
            GameStat::HpPercentage => 5.83,
            GameStat::ElementalMastery => 23.31,
            GameStat::EnergyRecharge => 6.48,
            GameStat::CritRate => 3.89,
            GameStat::CritDamage => 7.77,
            GameStat::PhysicalBonus => 58.3 / 8.0,
            GameStat::GeoBonus => 58.3 / 8.0,
            GameStat::ElectroBonus => 58.3 / 8.0,
            GameStat::AnemoBonus => 58.3 / 8.0,
            GameStat::DendroBonus => 58.3 / 8.0,
            GameStat::CryoBonus => 58.3 / 8.0,
            GameStat::HydroBonus => 58.3 / 8.0,
            GameStat::PyroBonus => 58.3 / 8.0,
            GameStat::HealingBonus => 35.9 / 8.0,
        }
    }
}
