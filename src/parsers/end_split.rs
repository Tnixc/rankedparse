use crate::match_record::{Duel, PlayerTimeline, TimelineType};
use crate::types::Millisec;

fn player_end_split(timeline: &[PlayerTimeline]) -> Option<Millisec> {
    let enter = timeline
        .iter()
        .find(|t| t.timeline_type == TimelineType::StoryEnterTheEnd)?;
    let kill = timeline
        .iter()
        .find(|t| t.timeline_type == TimelineType::EndKillDragon)?;

    Some(Millisec(enter.time.0.abs_diff(kill.time.0) as i128))
}

impl Duel {
    pub fn end_split(&self) -> (Option<Millisec>, Option<Millisec>) {
        (
            player_end_split(&self.timelines.0),
            player_end_split(&self.timelines.1),
        )
    }
}
