use crate::match_record::MatchRecord;
use crate::types::{Division, Millisec, NUM_DIVISIONS, Seconds};
use serde::Serialize;
use std::collections::HashMap;

const BUCKET_WIDTH: Millisec = Millisec(Seconds(60).to_ms());
const MAX_TIME: Millisec = Millisec(Seconds(120 * 60).to_ms()); // 2 hours
const NUM_BUCKETS: usize = (MAX_TIME.0 / BUCKET_WIDTH.0) as usize;

struct PlayerAccum {
    total: Millisec,
    count: u64,
    div_idx: usize,
}

pub struct CompletionTimeStats {
    players: HashMap<String, PlayerAccum>,
    completion_buckets: [[u64; NUM_BUCKETS]; NUM_DIVISIONS],
}

#[derive(Serialize)]
struct BucketEntry {
    minute: usize,
    completions: u64,
    players: u64,
}

#[derive(Serialize)]
struct CompletionOutput {
    total_completions: u64,
    total_players: usize,
    buckets: Vec<BucketEntry>,
}

impl CompletionTimeStats {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            completion_buckets: [[0; NUM_BUCKETS]; NUM_DIVISIONS],
        }
    }

    fn bucket_idx(time: Millisec) -> usize {
        (time.0 / BUCKET_WIDTH.0).clamp(0, NUM_BUCKETS as i128 - 1) as usize
    }

    pub fn feed(&mut self, record: &MatchRecord, divisions: &HashMap<&str, Division>) {
        for completion in &record.completions {
            if let Some(&div) = divisions.get(completion.uuid.as_str()) {
                self.completion_buckets[div.index()][Self::bucket_idx(completion.time)] += 1;
                let entry = self
                    .players
                    .entry(completion.uuid.clone())
                    .or_insert(PlayerAccum {
                        total: Millisec(0),
                        count: 0,
                        div_idx: div.index(),
                    });
                entry.total += completion.time;
                entry.count += 1;
                entry.div_idx = div.index();
            }
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        for d in 0..NUM_DIVISIONS {
            for i in 0..NUM_BUCKETS {
                self.completion_buckets[d][i] += other.completion_buckets[d][i];
            }
        }
        for (uuid, other_accum) in other.players {
            let entry = self.players.entry(uuid).or_insert(PlayerAccum {
                total: Millisec(0),
                count: 0,
                div_idx: other_accum.div_idx,
            });
            entry.total += other_accum.total;
            entry.count += other_accum.count;
            entry.div_idx = other_accum.div_idx;
        }
        self
    }

    fn division_json(&self, div_idx: usize, players: &[&PlayerAccum]) -> serde_json::Value {
        let total_completions: u64 = self.completion_buckets[div_idx].iter().sum();

        let mut player_buckets = [0u64; NUM_BUCKETS];
        for p in players {
            let avg = Millisec(p.total.0 / p.count as i128);
            player_buckets[Self::bucket_idx(avg)] += 1;
        }

        let buckets: Vec<BucketEntry> = (0..NUM_BUCKETS)
            .filter(|&i| self.completion_buckets[div_idx][i] > 0 || player_buckets[i] > 0)
            .map(|i| BucketEntry {
                minute: i,
                completions: self.completion_buckets[div_idx][i],
                players: player_buckets[i],
            })
            .collect();

        serde_json::to_value(CompletionOutput {
            total_completions,
            total_players: players.len(),
            buckets,
        })
        .unwrap()
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut players_by_div: [Vec<&PlayerAccum>; NUM_DIVISIONS] =
            std::array::from_fn(|_| Vec::new());
        for p in self.players.values() {
            players_by_div[p.div_idx].push(p);
        }

        let mut map = serde_json::Map::new();
        for div in Division::ALL {
            map.insert(
                div.name().to_string(),
                self.division_json(div.index(), &players_by_div[div.index()]),
            );
        }
        serde_json::Value::Object(map)
    }
}
