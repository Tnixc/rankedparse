use derive_more::{Add, AddAssign, Display, Div, Sub, Sum};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign, Sub, Div, Display, Sum)]
pub struct Seconds(pub u128);

#[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign, Sub, Div, Display, Sum)]
pub struct Millisec(pub u128);

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
