use std::{collections::HashMap, cmp::Ordering};

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_results: &str) -> String {
    let matches: Vec<&str> = match_results.split("\n").collect();
    let mut table: HashMap<String, (u32, u32, u32, u32, u32)> = HashMap::new();

    for m in matches {
        let pieces: Vec<&str> = m.split(";").collect();
        if pieces.len() < 3 {
            continue;
        }

        let home = pieces.get(0).unwrap().clone();
        let away = pieces.get(1).unwrap().clone();
        let result = pieces.get(2).unwrap().clone();

        let mut home_entry = table
            .entry(home.to_string())
            .or_insert((0, 0, 0, 0, 0))
            .clone();
        let mut away_entry = table
            .entry(away.to_string())
            .or_insert((0, 0, 0, 0, 0))
            .clone();
        home_entry.0 += 1;
        away_entry.0 += 1;

        if result == "win" {
            home_entry.1 += 1;
            away_entry.3 += 1;
            home_entry.4 += 3;
        } else if result == "loss" {
            away_entry.1 += 1;
            home_entry.3 += 1;
            away_entry.4 += 3;
        } else {
            home_entry.2 += 1;
            away_entry.2 += 1;
            home_entry.4 += 1;
            away_entry.4 += 1;
        }

        table.insert(home.to_string(), home_entry);
        table.insert(away.to_string(), away_entry);
    }
    let mut result = HEADER.to_string();
    let mut sorted_table: Vec<(&String, &(u32, u32, u32, u32, u32))> = table.iter().collect();

    sorted_table.sort_unstable_by(|a, b| {
        match b.1.4.cmp(&a.1.4) {
            Ordering::Equal => a.0.cmp(&b.0),
            _ => b.1.4.cmp(&a.1.4)
        }
    });

    for (team, entry) in sorted_table.iter() {
        result.push_str(
            format!(
                "\n{:width$}|  {} |  {} |  {} |  {} |  {}",
                team,
                entry.0,
                entry.1,
                entry.2,
                entry.3,
                entry.4,
                width = 31
            )
            .as_str(),
        );
    }

    result
}
