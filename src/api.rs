use std::collections::HashMap;

use crate::stats::GameStat;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Profile(ProfileResponse),
    NonProfileResponse(NonProfileResponse),
}

type ProfileResponse = HashMap<CharId, Vec<ProfileAvatar>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NonProfileResponse {
    pub avatar_info_list: Vec<Avatar>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileAvatar {
    pub name: String,
    pub live: bool,
    pub avatar_data: Avatar,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub avatar_id: u64,
    pub equip_list: Vec<Equip>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equip {
    pub flat: Flat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Flat {
    Reliquary(Reliquary),
    Weapon(Weapon),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub weapon_stats: Value,
    pub name_text_map_hash: Hash,
}

type CharId = String;
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(untagged)]
pub enum Hash {
    String(String),
    U64(u64), // old builds may still use u64 in db
}

#[allow(dead_code)]
impl Hash {
    #[allow(dead_code)]
    pub fn str_if_u64(self) -> String {
        match self {
            Self::String(s) => s,
            Self::U64(u) => u.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reliquary {
    pub equip_type: String,
    pub name_text_map_hash: Hash,
    pub set_name_text_map_hash: Hash,
    pub reliquary_mainstat: MainStat,
    pub reliquary_substats: Vec<SubStat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MainStat {
    pub main_prop_id: GameStat,
    pub stat_value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubStat {
    pub append_prop_id: GameStat,
    pub stat_value: f64,
}
