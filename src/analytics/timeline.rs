use super::Collector;
use crate::match_record::{MatchRecord, TimelineType};
use std::collections::{HashMap, HashSet};
use std::fmt;

pub struct TimelineEventStats {
    /// Number of matches that contain each event type (each match counted at most once per type)
    matches_with: HashMap<TimelineType, u64>,
    total_matches: u64,
}

impl TimelineEventStats {
    pub fn new() -> Self {
        Self {
            matches_with: HashMap::new(),
            total_matches: 0,
        }
    }
}

impl Collector for TimelineEventStats {
    fn feed(&mut self, record: &MatchRecord) {
        self.total_matches += 1;
        let types: HashSet<TimelineType> =
            record.timelines.iter().map(|tl| tl.timeline_type).collect();
        for ty in types {
            *self.matches_with.entry(ty).or_insert(0) += 1;
        }
    }

    fn name(&self) -> &str {
        "Timeline Event Distribution"
    }
}

impl fmt::Display for TimelineEventStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.total_matches == 0 {
            return writeln!(f, "  No matches recorded.");
        }

        let mut entries: Vec<_> = self.matches_with.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1));

        writeln!(f, "  Total matches: {}", self.total_matches)?;
        writeln!(f, "  {:45} {:>8} {:>7}", "Event", "Matches", "%")?;
        writeln!(f, "  {}", "-".repeat(62))?;

        for (event, count) in &entries {
            let pct = (**count as f64 / self.total_matches as f64) * 100.0;
            writeln!(
                f,
                "  {:45} {:>8} {:>6.1}%",
                format!("{:?}", event),
                count,
                pct
            )?;
        }
        Ok(())
    }
}
