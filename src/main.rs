#[allow(unused_imports)]
use reqwest::{Error, header};
use std::collections::HashMap;
mod api;
mod datamine;
mod stats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let roll_map = stats::rolls::parse_csv_to_map("resources/gi/char_usefulroll_map.csv");
    let args = std::env::args().collect::<Vec<_>>();
    let url = match args.get(1) {
        Some(v) => v,
        _ => "https://enka.network/api/uid/618285856",
    };

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("TheSast-enka-substat-exporter"),
    );

    println!("GET {url}");
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        println!("{}", response.status());
        let json: api::Response = response.json().await?;
        let omnimap = datamine::load_omnimap("resources/gi/char_charid_texmaphash_map.json");
        let charid_to_char = omnimap
            .into_iter()
            .map(|(name, (id, _))| (id, name))
            .collect::<HashMap<_, _>>();

        let build_list: Vec<api::ProfileAvatar> = match json {
            api::Response::Profile(ref json) => json.values().flatten().cloned().collect(),
            api::Response::NonProfileResponse(ref json) => json
                .avatar_info_list
                .clone()
                .into_iter()
                .map(|x| api::ProfileAvatar {
                    name: String::from(""),
                    avatar_data: x,
                    live: true,
                })
                .collect(),
        };
        for build in build_list {
            let character_name = charid_to_char
                .get(&build.avatar_data.avatar_id)
                .expect("invalid character id");
            if match args.get(2) {
                Some(v) => build.name.starts_with(v),
                _ => true,
            } {
                let equips = build.avatar_data.equip_list;
                println!(
                    "{:<32} {character_name:<20} {:<25} {:?} ",
                    format!(
                        "{:?}",
                        equips
                            .iter()
                            .filter_map(|x| if let api::Flat::Reliquary(ref rel) = x.flat {
                                Some(
                                    (rel.reliquary_substats
                                        .iter()
                                        .filter(|x| match roll_map.get(character_name) {
                                            Some(v) => v.contains(&x.append_prop_id),
                                            None => false,
                                        })
                                        .map(|x| x.stat_value / f64::from(x.append_prop_id))
                                        .sum::<f64>()
                                        * 100.0)
                                        .round()
                                        / 100.0,
                                )
                            } else {
                                None
                            })
                            .collect::<Vec<_>>(),
                    ),
                    if build.live { "live" } else { &build.name },
                    roll_map.get(character_name),
                )
            }
        }
    } else {
        println!("{:?}", response);
    }

    Ok(())
}
