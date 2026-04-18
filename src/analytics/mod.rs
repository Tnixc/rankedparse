mod completion_times;
mod forfeits;
mod playing_times;
mod split_stats;
mod timeline;

pub use completion_times::CompletionTimeStats;
pub use forfeits::ForfeitStats;
pub use playing_times::TemporalStats;
pub use split_stats::SplitStats;
pub use timeline::TimelineEventStats;

use crate::match_record::MatchRecord;
use std::fmt;

pub trait Collector: fmt::Display {
    fn feed(&mut self, record: &MatchRecord);
    fn name(&self) -> &str;
}

pub struct Pipeline {
    collectors: Vec<Box<dyn Collector>>,
    errors: u64,
    records_fed: u64,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            collectors: Vec::new(),
            errors: 0,
            records_fed: 0,
        }
    }

    pub fn add<C: Collector + 'static>(mut self, collector: C) -> Self {
        self.collectors.push(Box::new(collector));
        self
    }

    pub fn feed(&mut self, record: &MatchRecord) {
        self.records_fed += 1;
        if record.players.len() != 2 {
            return;
        }
        for collector in &mut self.collectors {
            collector.feed(record);
        }
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn report(&self) {
        println!(
            "  Total records: {}, Parse errors: {}",
            self.records_fed, self.errors
        );
        println!();
        for collector in &self.collectors {
            println!("  --- {} ---", collector.name());
            println!("{collector}");
        }
    }
}
