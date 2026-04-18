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
