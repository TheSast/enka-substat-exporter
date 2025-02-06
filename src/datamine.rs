// Load TextMap JSON into a HashMap for lookup
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;

pub(crate) fn load_textmap(file_path: &str) -> HashMap<String, String> {
    let data = fs::read_to_string(file_path).expect("Failed to read TextMap JSON file");
    let json: Map<String, Value> = serde_json::from_str(&data).expect("Invalid JSON format");

    json.into_iter()
        .map(|(key, value)| (key, value.as_str().unwrap_or("").to_string()))
        .collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct AvatarExcelEntry {
    #[serde(rename = "id")]
    avatar_id: u64,
    #[serde(rename = "nameTextMapHash")]
    name_hash: u64,
}

pub(crate) fn load_avatar_config(file_path: &str) -> HashMap<String, String> {
    let data =
        std::fs::read_to_string(file_path).expect("Failed to read AvatarExcelConfigData.json");
    let json: Vec<AvatarExcelEntry> = serde_json::from_str(&data).expect("Invalid JSON format");

    json.into_iter()
        .map(|entry| (entry.avatar_id.to_string(), entry.name_hash.to_string()))
        .collect()
}
