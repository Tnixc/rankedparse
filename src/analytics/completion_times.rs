use super::Collector;
use crate::match_record::MatchRecord;
use crate::types::{Millisec, Seconds};
use std::collections::HashMap;
use std::fmt;

const BUCKET_WIDTH: Millisec = Millisec(Seconds(60).to_ms());
const MAX_TIME: Millisec = Millisec(Seconds(120 * 60).to_ms()); // 2 hours
const NUM_BUCKETS: usize = (MAX_TIME.0 / BUCKET_WIDTH.0) as usize;

struct PlayerAccum {
    total: Millisec,
    count: u64,
}

pub struct CompletionTimeStats {
    players: HashMap<String, PlayerAccum>,
    match_buckets: [u64; NUM_BUCKETS],
}

impl CompletionTimeStats {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            match_buckets: [0; NUM_BUCKETS],
        }
    }

    fn bucket_idx(time: Millisec) -> usize {
        (time.0 / BUCKET_WIDTH.0).clamp(0, NUM_BUCKETS as i128 - 1) as usize
    }
}

impl Collector for CompletionTimeStats {
    fn feed(&mut self, record: &MatchRecord) {
        for completion in &record.completions {
            self.match_buckets[Self::bucket_idx(completion.time)] += 1;
            let entry = self
                .players
                .entry(completion.uuid.clone())
                .or_insert(PlayerAccum {
                    total: Millisec(0),
                    count: 0,
                });
            entry.total += completion.time;
            entry.count += 1;
        }
    }

    fn name(&self) -> &str {
        "Completion Time"
    }
}

impl fmt::Display for CompletionTimeStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_completions: u64 = self.match_buckets.iter().sum();
        if total_completions == 0 {
            return writeln!(f, "  No completions recorded.");
        }

        let mut player_buckets = [0u64; NUM_BUCKETS];
        for p in self.players.values() {
            let avg = Millisec(p.total.0 / p.count as i128);
            player_buckets[Self::bucket_idx(avg)] += 1;
        }

        writeln!(
            f,
            "  {:>8}  {:>10}  {:>10}",
            "Minute", "Completions", "Players"
        )?;
        writeln!(f, "  {}", "-".repeat(32))?;
        for i in 0..NUM_BUCKETS {
            let m = self.match_buckets[i];
            let p = player_buckets[i];
            if m == 0 && p == 0 {
                continue;
            }
            let label = if i == NUM_BUCKETS - 1 {
                format!("{}+", i)
            } else {
                format!("{}-{}", i, i + 1)
            };
            writeln!(f, "  {:>8}  {:>10}  {:>10}", label, m, p)?;
        }
        writeln!(
            f,
            "  {:>8}  {:>10}  {:>10}",
            "Total",
            total_completions,
            self.players.len()
        )?;
        Ok(())
    }
}
