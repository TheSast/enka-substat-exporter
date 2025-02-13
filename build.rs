mod src;
mod req {
    pub use reqwest::blocking::get;
}
use src::datamine;
use std::collections::BTreeMap;
use std::fs::exists;
use std::ops::Not;
use std::process::Command;
use std::{env, error, fs};

const DATA: [&str; 2] = [
    "resources/gi/char_charid_texmaphash_map.json",
    "resources/gi/char_usefulroll_map.csv",
];
fn main() -> Result<(), Box<dyn error::Error>> {
    match env::var("UPDATE_DATAMINE") {
        Ok(v) if v == "true" => {
            let textmap = datamine::textmap(
                &req::get("https://gitlab.com/Dimbreath/AnimeGameData/-/raw/master/TextMap/TextMapEN.json")
                    .expect("Failed to download data")
                    .text()
                    .unwrap()
            )
                .unwrap();
            let avatars = datamine::avatar_config(
                &req::get("https://gitlab.com/Dimbreath/AnimeGameData/-/raw/master/ExcelBinOutput/AvatarExcelConfigData.json")
                        .expect("Failed to download data")
                        .text()
                        .unwrap()
            )
                .unwrap();
            let processed_data = avatars
                .into_iter()
                .map(|(id, hash)| {
                    (
                        textmap.get(&hash).expect("invalid textmap hash"),
                        (id, hash),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            fs::write(
                DATA[0],
                serde_json::to_string_pretty(&processed_data).unwrap(),
            )
            .expect("Failed to write JSON file");
            println!("cargo:rerun-if-env-changed=UPDATE_DATAMINE");
        }
        _ => {
            if DATA.iter().any(|x| matches!(exists(x), Ok(true))).not() {
                eprintln!("Missing data, please run ./update_local_data.sh");
                std::process::exit(1);
            };
        }
    }
    Ok(())
}
