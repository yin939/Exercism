#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const DAY_MINS: i32 = 24 * 60;

fn helper(all_mins: i32) -> (i32, i32) {
    // div_euclid 欧几里得算法 除法
    // rem_euclid 欧几里得算法 取模
    if all_mins < 0 {
        return (
            (DAY_MINS + all_mins).div_euclid(60).rem_euclid(24),
            (DAY_MINS + all_mins).rem_euclid(60),
        );
    } else {
        return (
            all_mins.div_euclid(60).rem_euclid(24),
            all_mins.rem_euclid(60),
        );
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let all_mins = (hours * 60 + minutes) % DAY_MINS;
        let (hours, minutes) = helper(all_mins);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let all_mins = self.hours * 60 + self.minutes;

        let (hours, minutes) = helper(all_mins + (minutes % DAY_MINS));

        Clock { hours, minutes }
    }

    pub fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
