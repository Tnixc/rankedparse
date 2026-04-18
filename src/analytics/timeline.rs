use crate::match_record::{MatchRecord, TimelineType};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

pub struct TimelineEventStats {
    matches_with: HashMap<TimelineType, u64>,
    total_matches: u64,
}

#[derive(Serialize)]
struct EventEntry {
    event: TimelineType,
    count: u64,
}

impl TimelineEventStats {
    pub fn new() -> Self {
        Self {
            matches_with: HashMap::new(),
            total_matches: 0,
        }
    }

    pub fn feed(&mut self, record: &MatchRecord) {
        self.total_matches += 1;
        let types: HashSet<TimelineType> =
            record.timelines.iter().map(|tl| tl.timeline_type).collect();
        for ty in types {
            *self.matches_with.entry(ty).or_insert(0) += 1;
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.total_matches += other.total_matches;
        for (ty, count) in other.matches_with {
            *self.matches_with.entry(ty).or_insert(0) += count;
        }
        self
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut entries: Vec<EventEntry> = self
            .matches_with
            .iter()
            .map(|(&event, &count)| EventEntry { event, count })
            .collect();
        entries.sort_by(|a, b| b.count.cmp(&a.count));

        serde_json::json!({
            "total_matches": self.total_matches,
            "events": serde_json::to_value(entries).unwrap(),
        })
    }
}
