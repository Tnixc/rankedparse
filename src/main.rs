mod analytics;
mod match_record;
mod types;

use analytics::SeasonData;
use match_record::MatchRecord;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

fn main() {
    let mut season_stats: Vec<(usize, SeasonData)> = (1..=10)
        .into_par_iter()
        .map(|i| {
            let start = Instant::now();

            let path = format!("data/S{i}.jsonl");
            let file = File::open(&path).expect("failed to open data file");
            let reader = BufReader::new(file);

            let mut data = SeasonData::new();

            for line in reader.lines() {
                let Some(line) = line.ok() else { continue };
                match serde_json::from_str::<MatchRecord>(&line) {
                    Ok(record) => data.feed(&record),
                    Err(_) => data.record_error(),
                }
            }

            let dt = Instant::now().duration_since(start);
            eprintln!("Season {i} took {dt:?}");

            (i, data)
        })
        .collect();

    season_stats.sort_by_key(|(i, _)| *i);

    let mut out =
        File::create("output/analytics.jsonl").expect("failed to create output/analytics.jsonl");

    let mut aggregate = SeasonData::new();
    for (i, stats) in season_stats {
        writeln!(out, "{}", stats.to_json(&i.to_string())).unwrap();
        aggregate = aggregate.merge(stats);
    }
    writeln!(out, "{}", aggregate.to_json("all")).unwrap();

    eprintln!("Wrote output/analytics.jsonl");
}
