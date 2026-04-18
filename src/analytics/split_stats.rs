use crate::match_record::{MatchRecord, Timeline, TimelineType};
use crate::types::{Division, Millisec, NUM_DIVISIONS};
use serde::Serialize;
use std::collections::HashMap;

struct SplitDef {
    name: &'static str,
    from: TimelineType,
    to: TimelineType,
}

const SPLITS: &[SplitDef] = &[
    SplitDef {
        name: "Overworld",
        from: TimelineType::StoryRoot,
        to: TimelineType::StoryEnterTheNether,
    },
    SplitDef {
        name: "Nether Enter -> Bastion",
        from: TimelineType::StoryEnterTheNether,
        to: TimelineType::NetherFindBastion,
    },
    SplitDef {
        name: "Bastion -> Fortress",
        from: TimelineType::NetherFindBastion,
        to: TimelineType::NetherFindFortress,
    },
    SplitDef {
        name: "Stronghold",
        from: TimelineType::StoryFollowEnderEye,
        to: TimelineType::ProjecteloBlindTravel,
    },
    SplitDef {
        name: "End",
        from: TimelineType::EndRoot,
        to: TimelineType::EndKillDragon,
    },
    SplitDef {
        name: "Dragon Kill -> Complete",
        from: TimelineType::EndKillDragon,
        to: TimelineType::ProjecteloComplete,
    },
];

struct SplitAccum {
    total_ms: i128,
    count: u64,
}

pub struct SplitStats {
    accums: [[SplitAccum; SPLITS.len()]; NUM_DIVISIONS],
}

#[derive(Serialize)]
struct SplitEntry {
    name: &'static str,
    avg_ms: i128,
    count: u64,
}

impl SplitStats {
    pub fn new() -> Self {
        Self {
            accums: std::array::from_fn(|_| {
                std::array::from_fn(|_| SplitAccum {
                    total_ms: 0,
                    count: 0,
                })
            }),
        }
    }

    pub fn feed(&mut self, record: &MatchRecord, divisions: &HashMap<&str, Division>) {
        let mut by_player: HashMap<&str, Vec<&Timeline>> = HashMap::new();
        for tl in &record.timelines {
            by_player.entry(&tl.uuid).or_default().push(tl);
        }
        for (uuid, events) in &by_player {
            if let Some(&div) = divisions.get(*uuid) {
                process_player(events, &mut self.accums[div.index()]);
            }
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        for d in 0..NUM_DIVISIONS {
            for s in 0..SPLITS.len() {
                self.accums[d][s].total_ms += other.accums[d][s].total_ms;
                self.accums[d][s].count += other.accums[d][s].count;
            }
        }
        self
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for div in Division::ALL {
            let entries: Vec<SplitEntry> = SPLITS
                .iter()
                .zip(self.accums[div.index()].iter())
                .map(|(split, acc)| SplitEntry {
                    name: split.name,
                    avg_ms: if acc.count > 0 {
                        acc.total_ms / acc.count as i128
                    } else {
                        0
                    },
                    count: acc.count,
                })
                .collect();
            map.insert(
                div.name().to_string(),
                serde_json::to_value(entries).unwrap(),
            );
        }
        serde_json::Value::Object(map)
    }
}

fn find_time(events: &[&Timeline], ty: TimelineType) -> Option<Millisec> {
    events
        .iter()
        .find(|t| t.timeline_type == ty)
        .map(|t| t.time)
}

fn process_player(events: &[&Timeline], accums: &mut [SplitAccum]) {
    for (i, split) in SPLITS.iter().enumerate() {
        if let (Some(from), Some(to)) = (find_time(events, split.from), find_time(events, split.to))
        {
            let delta = (to.0 - from.0).abs();
            accums[i].total_ms += delta;
            accums[i].count += 1;
        }
    }
}
