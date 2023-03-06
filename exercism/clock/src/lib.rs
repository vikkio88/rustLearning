use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hour, minutes) = self.hour_minute();
        write!(f, "{:0>2}:{:0>2}", hour, minutes)
    }
}

const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_HOURS: i32 = 60;
const DAY_MINUTES: i32 = HOURS_IN_DAY * MINUTES_IN_HOURS;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let corrected_minutes = (DAY_MINUTES + minutes) % DAY_MINUTES;
        let minutes_hours = ((HOURS_IN_DAY + hours) % HOURS_IN_DAY) * MINUTES_IN_HOURS;

        Clock {
            minutes: (corrected_minutes + minutes_hours) % DAY_MINUTES,
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        let new_minutes = (DAY_MINUTES + self.minutes + minutes) % DAY_MINUTES;
        self.minutes = new_minutes;
        self
    }

    pub fn hour_minute(&self) -> (i32, i32) {
        let hours = self.minutes / MINUTES_IN_HOURS;
        let remaining = self.minutes - (hours * MINUTES_IN_HOURS);
        (hours, remaining)
    }
}
