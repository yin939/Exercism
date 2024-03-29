#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const DAY_MINS: i32 = 24 * 60;

/// return (hours, minutes)
fn helper(all_mins: i32) -> (i32, i32) {
    // div_euclid 欧几里得算法 除法
    // rem_euclid 欧几里得算法 取模
    if all_mins < 0 {
        return (
            (DAY_MINS + all_mins).div_euclid(60).rem_euclid(24),
            (DAY_MINS + all_mins).rem_euclid(60),
        );
    }

    return (
        all_mins.div_euclid(60).rem_euclid(24),
        all_mins.rem_euclid(60),
    );
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = (hours * 60 + minutes).rem_euclid(DAY_MINS);
        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        Clock { minutes }
    }

    pub fn to_string(&self) -> String {
        let (hours, minutes) = helper(self.minutes);
        format!("{:0>2}:{:0>2}", hours, minutes)
    }
}
