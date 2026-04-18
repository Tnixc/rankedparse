use crate::match_record::{Duel, MatchRecord, PlayerTimeline};

impl MatchRecord {
    pub fn into_duel(self) -> Option<Duel> {
        if self.players.len() != 2 {
            return None;
        }

        let mut p0 = Vec::new();
        let mut p1 = Vec::new();

        for event in &self.timelines {
            let pt = PlayerTimeline {
                time: event.time,
                timeline_type: event.timeline_type,
            };
            if event.uuid == self.players[0].uuid {
                p0.push(pt);
            } else if event.uuid == self.players[1].uuid {
                p1.push(pt);
            }
        }

        Some(Duel {
            record: self,
            timelines: (p0, p1),
        })
    }
}
