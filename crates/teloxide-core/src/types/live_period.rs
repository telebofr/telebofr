#![allow(clippy::from_over_into)]

use serde::{Deserialize, Serialize};

use crate::types::Seconds;

/// Period in seconds for which the location can be updated, should be
/// between 60 and 86400, or 0x7FFFFFFF for live locations that can be
/// edited indefinitely.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize)]
#[serde(untagged)]
pub enum LivePeriod {
    Timeframe(Seconds),
    Indefinite,
}

impl LivePeriod {
    pub fn timeframe(&self) -> Option<Seconds> {
        self.try_into().ok()
    }

    pub fn is_indefinite(&self) -> bool {
        matches!(self, Self::Indefinite)
    }

    pub fn from_u32(seconds: u32) -> Self {
        LivePeriod::Timeframe(Seconds::from_seconds(seconds))
    }

    pub fn from_seconds(seconds: Seconds) -> Self {
        seconds.into()
    }
}

impl TryInto<Seconds> for LivePeriod {
    type Error = &'static str;

    fn try_into(self) -> Result<Seconds, Self::Error> {
        match self {
            LivePeriod::Timeframe(v) => Ok(v),
            LivePeriod::Indefinite => Err("indefinite live period"),
        }
    }
}

impl TryInto<Seconds> for &LivePeriod {
    type Error = &'static str;

    fn try_into(self) -> Result<Seconds, Self::Error> {
        match self {
            LivePeriod::Timeframe(v) => Ok(*v),
            LivePeriod::Indefinite => Err("indefinite live period"),
        }
    }
}

impl Into<LivePeriod> for Seconds {
    fn into(self) -> LivePeriod {
        LivePeriod::from_seconds(self)
    }
}

impl Into<LivePeriod> for u32 {
    fn into(self) -> LivePeriod {
        LivePeriod::from_u32(self)
    }
}

impl<'de> Deserialize<'de> for LivePeriod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;

        if value == 0x7FFFFFFF {
            Ok(LivePeriod::Indefinite)
        } else {
            Ok(LivePeriod::Timeframe(Seconds::from_seconds(value)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Struct {
        live_period: Option<LivePeriod>,
    }

    #[test]
    fn deserialize_indefinite() {
        let json = r#"{"live_period": 2147483647}"#; // 0x7FFFFFFF
        let expected = LivePeriod::Indefinite;
        let Struct { live_period } = serde_json::from_str(json).unwrap();
        assert_eq!(live_period, Some(expected));
    }

    #[test]
    fn deserialize_900() {
        let json = r#"{"live_period": 900}"#;
        let expected = LivePeriod::from_u32(900);
        let Struct { live_period } = serde_json::from_str(json).unwrap();
        assert_eq!(live_period, Some(expected));
    }
}
