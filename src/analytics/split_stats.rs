use super::Collector;
use crate::match_record::{MatchRecord, Timeline, TimelineType};
use crate::types::Millisec;
use std::collections::HashMap;
use std::fmt;

struct Split {
    name: &'static str,
    from: TimelineType,
    to: TimelineType,
}

const SPLITS: &[Split] = &[
    Split {
        name: "Overworld",
        from: TimelineType::StoryRoot,
        to: TimelineType::StoryEnterTheNether,
    },
    Split {
        name: "Nether Enter -> Bastion",
        from: TimelineType::StoryEnterTheNether,
        to: TimelineType::NetherFindBastion,
    },
    Split {
        name: "Bastion -> Fortress",
        from: TimelineType::NetherFindBastion,
        to: TimelineType::NetherFindFortress,
    },
    Split {
        name: "Stronghold",
        from: TimelineType::StoryFollowEnderEye,
        to: TimelineType::ProjecteloBlindTravel,
    },
    Split {
        name: "End",
        from: TimelineType::EndRoot,
        to: TimelineType::EndKillDragon,
    },
    Split {
        name: "Dragon Kill -> Complete",
        from: TimelineType::EndKillDragon,
        to: TimelineType::ProjecteloComplete,
    },
];

struct SplitAccum {
    total: Millisec,
    count: u64,
}

pub struct SplitStats {
    accums: Vec<SplitAccum>,
}

impl SplitStats {
    pub fn new() -> Self {
        Self {
            accums: SPLITS
                .iter()
                .map(|_| SplitAccum {
                    total: Millisec(0),
                    count: 0,
                })
                .collect(),
        }
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
            let delta = Millisec((to.0 - from.0).abs());
            accums[i].total += delta;
            accums[i].count += 1;
        }
    }
}

impl Collector for SplitStats {
    fn feed(&mut self, record: &MatchRecord) {

        let mut by_player: HashMap<&str, Vec<&Timeline>> = HashMap::new();
        for tl in &record.timelines {
            by_player.entry(&tl.uuid).or_default().push(tl);
        }

        for events in by_player.values() {
            process_player(events, &mut self.accums);
        }
    }

    fn name(&self) -> &str {
        "Split Times"
    }
}

impl fmt::Display for SplitStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  {:30} {:>10} {:>10}", "Split", "Avg", "Count")?;
        writeln!(f, "  {}", "-".repeat(52))?;

        for (i, split) in SPLITS.iter().enumerate() {
            let acc = &self.accums[i];
            if acc.count > 0 {
                let avg = Millisec(acc.total.0 / acc.count as i128);
                writeln!(
                    f,
                    "  {:30} {:>10} {:>10}",
                    split.name,
                    avg.format_human(),
                    acc.count
                )?;
            } else {
                writeln!(f, "  {:30} {:>10} {:>10}", split.name, "-", 0)?;
            }
        }
        Ok(())
    }
}
