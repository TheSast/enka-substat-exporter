use crate::stats::GameStat;
use std::collections::HashMap;
use std::fs;

pub(crate) fn parse_csv_to_map(file_path: &str) -> HashMap<String, Vec<GameStat>> {
    let csv_data = fs::read_to_string(file_path).expect("Failed to read TextMap JSON file");
    let mut map: HashMap<String, Vec<GameStat>> = HashMap::new();
    let lines: Vec<&str> = csv_data.lines().collect();

    let headers = [
        GameStat::Hp,
        GameStat::HpPercentage,
        GameStat::Def,
        GameStat::DefPercentage,
        GameStat::Atk,
        GameStat::AtkPercentage,
        GameStat::CritRate,
        GameStat::CritDamage,
        GameStat::ElementalMastery,
        GameStat::EnergyRecharge,
    ];

    for line in lines.iter().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();
        if columns.len() < 11 {
            continue;
        }

        let character = columns[0].trim().to_string();
        if character.is_empty() {
            continue;
        }

        let mut stats = Vec::new();
        for (i, &stat) in headers.iter().enumerate() {
            if let Some(&value) = columns.get(i + 1) {
                if value.trim() == "TRUE" {
                    stats.push(stat);
                }
            }
        }

        map.insert(character, stats);
    }
    map
}
