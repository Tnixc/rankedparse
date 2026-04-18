mod completion_times;
mod forfeits;
mod playing_times;
mod split_stats;
mod timeline;

pub use completion_times::CompletionTimeStats;
pub use forfeits::ForfeitStats;
pub use split_stats::SplitStats;
pub use timeline::TimelineEventStats;

use crate::match_record::MatchRecord;
use crate::types::Division;
use std::collections::HashMap;

pub struct SeasonData {
    pub forfeits: ForfeitStats,
    pub timeline: TimelineEventStats,
    pub splits: SplitStats,
    pub completions: CompletionTimeStats,
    pub records: u64,
    pub errors: u64,
}

impl SeasonData {
    pub fn new() -> Self {
        Self {
            forfeits: ForfeitStats::new(),
            timeline: TimelineEventStats::new(),
            splits: SplitStats::new(),
            completions: CompletionTimeStats::new(),
            records: 0,
            errors: 0,
        }
    }

    pub fn feed(&mut self, record: &MatchRecord) {
        self.records += 1;
        if record.players.len() != 2 {
            return;
        }

        let divisions: HashMap<&str, Division> = record
            .players
            .iter()
            .filter_map(|p| {
                p.elo_rate
                    .map(|elo| (p.uuid.as_str(), Division::from_elo(elo)))
            })
            .collect();

        self.forfeits.feed(record, &divisions);
        self.timeline.feed(record);
        self.splits.feed(record, &divisions);
        self.completions.feed(record, &divisions);
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            forfeits: self.forfeits.merge(other.forfeits),
            timeline: self.timeline.merge(other.timeline),
            splits: self.splits.merge(other.splits),
            completions: self.completions.merge(other.completions),
            records: self.records + other.records,
            errors: self.errors + other.errors,
        }
    }

    pub fn to_json(&self, season: &str) -> String {
        let obj = serde_json::json!({
            "season": season,
            "records": self.records,
            "errors": self.errors,
            "forfeits": self.forfeits.to_json(),
            "timeline": self.timeline.to_json(),
            "splits": self.splits.to_json(),
            "completion_times": self.completions.to_json(),
        });
        serde_json::to_string(&obj).unwrap()
    }
}
