mod src;
mod req {
    pub use reqwest::blocking::get;
}
use src::datamine;
use std::collections::BTreeMap;
use std::fs::exists;
use std::ops::Not;
use std::{env, error, fs};

const DATA: [&str; 2] = [
    "resources/gi/char_charid_texmaphash_map.json",
    "resources/gi/char_usefulroll_map.csv",
];
const URLS: [&str; 3] = [
    "https://gitlab.com/Dimbreath/AnimeGameData/-/raw/master/TextMap/TextMapEN.json",
    "https://gitlab.com/Dimbreath/AnimeGameData/-/raw/master/ExcelBinOutput/AvatarExcelConfigData.json",
    "https://docs.google.com/spreadsheets/d/1EBSxkR_fjX2kckY0G-lcZGCp5ugnDp83lWg2bxTgsL8/gviz/tq?tqx=out:csv&gid=163318763",
];
fn main() -> Result<(), Box<dyn error::Error>> {
    match env::var("UPDATE_DATA") {
        Ok(v) if v == "true" => {
            let textmap = datamine::textmap(
                &req::get(URLS[0])
                    .expect("Failed to download textmap data")
                    .text()
                    .unwrap(),
            )
            .unwrap();
            let avatars = datamine::avatar_config(
                &req::get(URLS[1])
                    .expect("Failed to download avatar data")
                    .text()
                    .unwrap(),
            )
            .unwrap();
            let processed_avatar_text_data = avatars
                .into_iter()
                .map(|(id, hash)| {
                    (
                        textmap.get(&hash).expect("invalid textmap hash"),
                        (if id == 10000007 { 10000005 } else { id }, hash), // HACK: FIXME: Hardcode
                                                                            // Aether for Traveller
                    )
                })
                .collect::<BTreeMap<_, _>>();
            fs::write(
                DATA[0],
                serde_json::to_string_pretty(&processed_avatar_text_data).unwrap(),
            )
            .expect("Failed to write JSON file");

            let char_usefulroll_map = req::get(URLS[2])
                .expect("Failed to download useful roll data")
                .text()
                .unwrap()
                .replace("\"", "")
                .lines()
                .map(|line| {
                    line.split(",")
                        .enumerate()
                        .filter(|&(n, _)| n == 11 || n >= 17)
                        .map(|(_, v)| v.to_owned())
                        .collect::<Vec<String>>()
                        .join(",")
                })
                .collect::<Vec<String>>()
                .join("\n");
            fs::write(DATA[1], &char_usefulroll_map).expect("Failed to write CSV file");
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
