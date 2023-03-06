use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

fn convert(hours: i32, minutes: i32) -> (i32, i32) {
    let mut new_hours = hours % 12;
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_minutes = minutes % 60;
        let mut new_hours = hours;

        if minutes > 60 {
            new_hours += minutes / 60;
        }

        Clock {
            hours: new_hours % 24,
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        let base = self.minutes + minutes;
        let new_minutes = base % 60;

        if minutes > 60 {
            self.hours += minutes / 60;
        }
        self.minutes = new_minutes;

        self
    }
}
