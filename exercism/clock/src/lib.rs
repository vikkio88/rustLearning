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

const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }

    pub fn hour_minute(&self) -> (i32, i32) {
        let hours = self.minutes / HOUR;
        let remaining = self.minutes - (hours * HOUR);
        (hours, remaining)
    }
}
