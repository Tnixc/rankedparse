use crate::match_record::MatchRecord;
use crate::types::{Division, NUM_DIVISIONS};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct ForfeitEntry {
    total: u64,
    forfeited: u64,
}

pub struct ForfeitStats {
    divisions: [ForfeitEntry; NUM_DIVISIONS],
}

impl ForfeitStats {
    pub fn new() -> Self {
        Self {
            divisions: std::array::from_fn(|_| ForfeitEntry {
                total: 0,
                forfeited: 0,
            }),
        }
    }

    pub fn feed(&mut self, record: &MatchRecord, divisions: &HashMap<&str, Division>) {
        for player in &record.players {
            if let Some(&div) = divisions.get(player.uuid.as_str()) {
                let entry = &mut self.divisions[div.index()];
                entry.total += 1;
                if record.forfeited {
                    entry.forfeited += 1;
                }
            }
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        for i in 0..NUM_DIVISIONS {
            self.divisions[i].total += other.divisions[i].total;
            self.divisions[i].forfeited += other.divisions[i].forfeited;
        }
        self
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for div in Division::ALL {
            let entry = &self.divisions[div.index()];
            map.insert(div.name().to_string(), serde_json::to_value(entry).unwrap());
        }
        serde_json::Value::Object(map)
    }
}
