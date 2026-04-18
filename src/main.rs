mod analytics;
mod match_record;
mod types;

use analytics::{CompletionTimeStats, ForfeitStats, Pipeline, SplitStats, TimelineEventStats};
use match_record::MatchRecord;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    (1..=10).into_par_iter().for_each(|i| {
        let start = Instant::now();
        println!("=== SEASON {} ===", i);

        let path = format!("data/S{i}.jsonl");
        let file = File::open(&path).expect("failed to open data file");
        let reader = BufReader::new(file);

        let mut pipeline = Pipeline::new()
            .add(ForfeitStats::new())
            .add(TimelineEventStats::new())
            .add(SplitStats::new())
            .add(CompletionTimeStats::new())
            // .add(TemporalStats::new())
            ;

        for line in reader.lines() {
            let Some(line) = line.ok() else { continue };
            match serde_json::from_str::<MatchRecord>(&line) {
                Ok(record) => pipeline.feed(&record),
                Err(_) => pipeline.record_error(),
            }
        }

        pipeline.report();

        let dt = Instant::now().duration_since(start);
        println!("  >>> Season {i} took {dt:?}");
        println!();
    })
}
