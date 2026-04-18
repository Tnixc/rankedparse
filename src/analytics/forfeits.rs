use super::Collector;
use crate::match_record::MatchRecord;
use std::fmt;

pub struct ForfeitStats {
    total: u64,
    forfeited: u64,
}

impl ForfeitStats {
    pub fn new() -> Self {
        Self {
            total: 0,
            forfeited: 0,
        }
    }
}

impl Collector for ForfeitStats {
    fn feed(&mut self, record: &MatchRecord) {
        self.total += 1;
        if record.forfeited {
            self.forfeited += 1;
        }
    }

    fn name(&self) -> &str {
        "Forfeit Statistics"
    }
}

impl fmt::Display for ForfeitStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.total == 0 {
            return writeln!(f, "  No matches recorded.");
        }
        let pct = (self.forfeited as f64 / self.total as f64) * 100.0;
        writeln!(
            f,
            "  Forfeited: {} / {} ({:.1}%)",
            self.forfeited, self.total, pct
        )
    }
}
