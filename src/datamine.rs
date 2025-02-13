use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

// pub(crate) fn load_textmap(
//     file_path: &str,
// ) -> Result<HashMap<String, String>, Result<(), Box<dyn std::error::Error>>> {
//     let data = fs::read_to_string(file_path).expect("Failed to read TextMap JSON file");
//     textmap(&data)
// }

#[allow(dead_code)]
pub(crate) fn textmap(
    data: &str,
) -> Result<HashMap<String, String>, Result<(), Box<dyn std::error::Error>>> {
    let json: HashMap<String, Value> = serde_json::from_str(data).expect("Invalid JSON format");

    Ok(json
        .into_iter()
        .map(|(key, value)| (key, value.as_str().unwrap_or("").to_string()))
        .collect())
}

#[derive(Serialize, Deserialize, Debug)]
struct AvatarExcelEntry {
    #[serde(rename = "id")]
    avatar_id: u64,
    #[serde(rename = "nameTextMapHash")]
    name_hash: u64,
}

// pub(crate) fn load_avatar_config(
//     file_path: &str,
// ) -> Result<HashMap<String, String>, Result<(), Box<dyn std::error::Error>>> {
//     let data =
//         std::fs::read_to_string(file_path).expect("Failed to read AvatarExcelConfigData.json");
//     avatar_config(&data)
// }

#[allow(dead_code)]
pub(crate) fn avatar_config(
    data: &str,
) -> Result<HashMap<u64, String>, Result<(), Box<dyn std::error::Error>>> {
    let json: Vec<AvatarExcelEntry> = serde_json::from_str(data).expect("Invalid JSON format");

    Ok(json
        .into_iter()
        .map(|entry| (entry.avatar_id, entry.name_hash.to_string()))
        .collect())
}

#[allow(dead_code)]
pub(crate) fn load_omnimap(file_path: &str) -> HashMap<String, (u64, String)> {
    let data = fs::read_to_string(file_path).expect("Failed to read omniMap JSON file");
    omnimap(&data)
}

#[allow(dead_code)]
pub(crate) fn omnimap(data: &str) -> HashMap<String, (u64, String)> {
    let json: HashMap<String, (u64, String)> =
        serde_json::from_str(data).expect("Invalid JSON format");
    json
    //     .into_iter()
    //     .map(|(name, (id, hash))| (name, (id, hash)))
    //     .collect()
}
