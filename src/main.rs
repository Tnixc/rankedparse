#[allow(dead_code)]
mod filters;
mod match_record;
mod parsers;
mod types;

use match_record::MatchRecord;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::types::{Millisec, Seconds};

fn main() {
    let path = std::env::args().nth(1).unwrap_or("data/S4.jsonl".into());
    let file = File::open(&path).expect("failed to open data file");
    let reader = BufReader::new(file);

    let mut errors = 0;
    let mut count = 0;
    let mut total_end_split = Millisec(0);

    let duels = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            serde_json::from_str::<MatchRecord>(&line)
                .inspect_err(|_| errors += 1)
                .ok()
        })
        .filter_map(|r| r.into_duel());

    let splits: Vec<Millisec> = duels
        .flat_map(|d| {
            let (a, b) = d.end_split();
            [a, b].into_iter().flatten()
        })
        .collect();

    let count = splits.len() as u128;
    let total: Millisec = splits.into_iter().sum();

    println!("parsed {count} end splits, {errors} errors");
    if count > 0 {
        println!("average end split: {}", Seconds::from(total / count));
    }
}
