use super::Collector;
use crate::match_record::MatchRecord;
use crate::types::Millisec;
use std::fmt;

pub struct CompletionTimeStats {
    total: Millisec,
    count: u64,
}

impl CompletionTimeStats {
    pub fn new() -> Self {
        Self {
            total: Millisec(0),
            count: 0,
        }
    }
}

impl Collector for CompletionTimeStats {
    fn feed(&mut self, record: &MatchRecord) {
        if let Some(result) = &record.result {
            self.total += result.time;
            self.count += 1;
        }
    }

    fn name(&self) -> &str {
        "Completion Time"
    }
}

impl fmt::Display for CompletionTimeStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.count == 0 {
            return writeln!(f, "  No completions recorded.");
        }
        let avg = Millisec(self.total.0 / self.count as i128);
        writeln!(
            f,
            "  Average: {} ({} completions)",
            avg.format_human(),
            self.count
        )
    }
}
