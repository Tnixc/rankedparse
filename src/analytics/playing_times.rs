use super::Collector;
use crate::match_record::MatchRecord;
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::collections::BTreeMap;
use std::fmt;

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

    fn weekday_name(idx: usize) -> &'static str {
        match idx {
            0 => "Mon",
            1 => "Tue",
            2 => "Wed",
            3 => "Thu",
            4 => "Fri",
            5 => "Sat",
            6 => "Sun",
            _ => unreachable!(),
        }
    }
}

impl Collector for TemporalStats {
    fn feed(&mut self, record: &MatchRecord) {
        let Some(dt) = DateTime::<Utc>::from_timestamp(record.date as i64, 0) else {
            return;
        };

        self.total += 1;
        *self.by_year.entry(dt.year()).or_insert(0) += 1;
        self.by_weekday[dt.weekday().num_days_from_monday() as usize] += 1;
        self.by_hour[dt.hour() as usize] += 1;
    }

    fn name(&self) -> &str {
        "Temporal Distribution"
    }
}

impl fmt::Display for TemporalStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.total == 0 {
            return writeln!(f, "  No matches recorded.");
        }

        writeln!(f, "  By Year:")?;
        for (year, count) in &self.by_year {
            let pct = (*count as f64 / self.total as f64) * 100.0;
            writeln!(f, "    {}: {:>8} ({:.1}%)", year, count, pct)?;
        }

        writeln!(f)?;
        writeln!(f, "  By Day of Week:")?;
        for (i, count) in self.by_weekday.iter().enumerate() {
            let pct = (*count as f64 / self.total as f64) * 100.0;
            writeln!(
                f,
                "    {}: {:>8} ({:.1}%)",
                Self::weekday_name(i),
                count,
                pct
            )?;
        }

        writeln!(f)?;
        writeln!(f, "  By Hour of Day (UTC):")?;
        for (hour, count) in self.by_hour.iter().enumerate() {
            let pct = (*count as f64 / self.total as f64) * 100.0;
            writeln!(f, "    {:02}:00: {:>8} ({:.1}%)", hour, count, pct)?;
        }

        Ok(())
    }
}
