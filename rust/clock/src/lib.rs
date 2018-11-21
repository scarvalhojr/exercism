use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

// Helper function while Euclidean modulo isn't available
// See: https://github.com/rust-lang/rust/issues/49048
fn mod_euc(dividend: i32, divisor: i32) -> i32 {
    ((dividend % divisor) + divisor) % divisor
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::build(hours * 60 + minutes)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::build(self.minutes + minutes)
    }

    fn build(minutes: i32) -> Self {
        Clock {
            minutes: mod_euc(minutes, 24 * 60),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
