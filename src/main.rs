#[allow(dead_code)]
mod filters;
mod match_record;
mod parsers;
mod types;

use match_record::MatchRecord;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::types::{Millisec, Seconds};

fn main() {
    for i in 1..=10 {
        let start = Instant::now();

        println!("SEASON {i}");
        let path = std::env::args()
            .nth(1)
            .unwrap_or(format!("data/S{i}.jsonl").into());
        let file = File::open(&path).expect("failed to open data file");
        let reader = BufReader::new(file);

        let mut errors = 0;

        let duels = reader
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| {
                serde_json::from_str::<MatchRecord>(&line)
                    .inspect_err(|e| {
                        errors += 1;
                        if errors < 1 {
                            dbg!(line);
                            dbg!(e);
                        }
                    })
                    .ok()
            })
            .filter_map(|r| r.into_duel());

        let splits: Vec<Millisec> = duels
            .flat_map(|d| {
                let (a, b) = d.end_split();
                [a, b].into_iter().flatten()
            })
            .collect();

        let count = splits.len() as i128;
        let total: Millisec = splits.into_iter().sum();

        println!("parsed {count} end splits, {errors} errors");
        if count > 0 {
            println!("average end split: {}", Seconds::from(total / count));
        }

        let dt = Instant::now().duration_since(start);
        println!(">>> Iteration took {:?}", dt);
    }
}
