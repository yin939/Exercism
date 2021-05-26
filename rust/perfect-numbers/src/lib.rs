#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let mut sum = 0_u64;
    for i in 1..=(num / 2) {
        if num % i == 0 {
            sum += i;
        }
    }

    match sum {
        sum if sum < num => Some(Classification::Deficient),
        sum if sum == num => Some(Classification::Perfect),
        _ => Some(Classification::Abundant),
    }
}
