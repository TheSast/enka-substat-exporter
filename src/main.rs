#[allow(unused_imports)]
use reqwest::{Error, header};
mod api;
mod datamine;
mod rolls;
mod stats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let roll_map = rolls::parse_csv_to_map("CharacterUsefulRollMap.csv");
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
        let json: api::Response = response.json().await?;
        let textmap = datamine::load_textmap("TextMapEN.json");
        let avatar_map = datamine::load_avatar_config("AvatarExcelConfigData.json");

        match json {
            api::Response::NonProfileResponse(json) => {
                for avatar in json.avatar_info_list {
                    let character_name = textmap
                        .get(
                            avatar_map
                                .get(&avatar.avatar_id.to_string())
                                .expect("invalid character id"),
                        )
                        .expect("invalid textmap hash");
                    println!("{character_name:<50} {:?}", roll_map.get(character_name));
                    let equips = avatar.equip_list;
                    println!(
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
                            .collect::<Vec<_>>()
                    )
                }
            }
            api::Response::Profile(json) => {
                for (char_id, builds) in json {
                    let character_name = textmap
                        .get(avatar_map.get(&char_id).expect("invalid character id"))
                        .expect("invalid textmap hash");
                    println!("{character_name:<50} {:?}", roll_map.get(character_name));
                    for build in builds {
                        if build
                            .name
                            .starts_with(args.get(2).unwrap_or(&String::from("ese:")))
                        {
                            let equips = build.avatar_data.equip_list;
                            println!(
                                "{} {:?}",
                                build.name,
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
                                    .collect::<Vec<_>>()
                            )
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}
