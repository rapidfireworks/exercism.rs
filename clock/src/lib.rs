use std::fmt::Display;

use chrono::{self, Duration, NaiveTime};

#[derive(Debug, PartialEq)]
pub struct Clock {
  time: NaiveTime,
}

impl Clock {
  pub fn new(hours: i64, minutes: i64) -> Self {
    Clock {
      time: NaiveTime::from_hms(0, 0, 0) + Duration::hours(hours) + Duration::minutes(minutes),
    }
  }

  pub fn add_minutes(&self, minutes: i64) -> Self {
    Clock {
      time: self.time + Duration::minutes(minutes),
    }
  }
}

impl Display for Clock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.time.format("%H:%M").to_string())
  }
}
