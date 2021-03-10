use std::fmt::{Debug, Display};

use chrono::{self, Duration, NaiveTime, Timelike};

pub struct Clock {
  time: NaiveTime,
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    let day = 24 * 60 * 60;
    let mut secs = ((hours * 60 + minutes) * 60) % day;
    if secs < 0 {
      secs += day;
    }
    Clock {
      time: NaiveTime::from_num_seconds_from_midnight(secs as u32, 0),
    }
  }

  pub fn add_minutes(&self, minutes: i32) -> Self {
    Clock {
      time: self.time + Duration::minutes(minutes.into()),
    }
  }
}

impl Display for Clock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:02}:{:02}", self.time.hour(), self.time.minute())
  }
}

impl Debug for Clock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:02}:{:02}", self.time.hour(), self.time.minute())
  }
}

impl PartialEq for Clock {
  fn eq(&self, other: &Self) -> bool {
    self.time == other.time
  }
}
