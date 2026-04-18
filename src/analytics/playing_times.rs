use crate::match_record::MatchRecord;
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::collections::BTreeMap;

pub struct TemporalStats {
    by_year: BTreeMap<i32, u64>,
    by_weekday: [u64; 7], // Mon=0 .. Sun=6
    by_hour: [u64; 24],
    total: u64,
}

impl TemporalStats {
    pub fn new() -> Self {
        Self {
            by_year: BTreeMap::new(),
            by_weekday: [0; 7],
            by_hour: [0; 24],
            total: 0,
        }
    }

    pub fn feed(&mut self, record: &MatchRecord) {
        let Some(dt) = DateTime::<Utc>::from_timestamp(record.date as i64, 0) else {
            return;
        };

        self.total += 1;
        *self.by_year.entry(dt.year()).or_insert(0) += 1;
        self.by_weekday[dt.weekday().num_days_from_monday() as usize] += 1;
        self.by_hour[dt.hour() as usize] += 1;
    }
}
