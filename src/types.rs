use derive_more::{Add, AddAssign, Display, Div, Sub, Sum};
use serde::Deserialize;

#[derive(
    Deserialize,
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    AddAssign,
    Sub,
    Div,
    Display,
    Sum,
)]
pub struct Seconds(pub i128);

#[derive(
    Deserialize,
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    AddAssign,
    Sub,
    Div,
    Display,
    Sum,
)]
pub struct Millisec(pub i128);

impl Seconds {
    pub const fn to_ms(self) -> i128 {
        self.0 * 1000
    }
}

impl From<Seconds> for Millisec {
    fn from(s: Seconds) -> Self {
        Millisec(s.0 * 1000)
    }
}

impl From<Millisec> for Seconds {
    fn from(m: Millisec) -> Self {
        Seconds(m.0 / 1000)
    }
}

impl Millisec {
    pub fn format_human(self) -> String {
        let total_secs = self.0 / 1000;
        let millis = self.0 % 1000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        if mins > 0 {
            format!("{}m {:02}.{:03}s", mins, secs, millis)
        } else {
            format!("{}.{:03}s", secs, millis)
        }
    }
}
pub const NUM_DIVISIONS: usize = 6;

///```md
/// 0 ~ 599	Coal	Lowest rank in the game.
/// 600 ~ 899	Iron	Unlocks Ruined Portals as a seed type.
/// 900 ~ 1199	Gold	Most common rank.
/// 1200 ~ 1499	Emerald	Unlocks Buried Treasures as a seed type.
/// 1500 ~ 1999	Diamond	This roughly represents the top 5% of players.
/// 2000+	Netherite	This roughly represents the top 0.5% of players.
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Division {
    Coal,
    Iron,
    Gold,
    Emerald,
    Diamond,
    Netherite,
}

impl Division {
    pub const ALL: [Division; NUM_DIVISIONS] = [
        Division::Coal,
        Division::Iron,
        Division::Gold,
        Division::Emerald,
        Division::Diamond,
        Division::Netherite,
    ];

    pub fn from_elo(rank: u32) -> Self {
        match rank {
            0..=599 => Self::Coal,
            600..=899 => Self::Iron,
            900..=1199 => Self::Gold,
            1200..=1499 => Self::Emerald,
            1500..=1999 => Self::Diamond,
            2000..=u32::MAX => Self::Netherite,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Coal => "coal",
            Self::Iron => "iron",
            Self::Gold => "gold",
            Self::Emerald => "emerald",
            Self::Diamond => "diamond",
            Self::Netherite => "netherite",
        }
    }
}
