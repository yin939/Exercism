use std::cmp;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // ·goal· is accumulated by gcd(capacity_1, capacity_2)
    if goal % gcd(capacity_1, capacity_2) != 0 {
        return None;
    }
    match start_bucket {
        &Bucket::One => {
            if capacity_2 == goal {
                return Some(BucketStats {
                    moves: 2,
                    goal_bucket: Bucket::Two,
                    other_bucket: capacity_1,
                });
            }
        }
        &Bucket::Two => {
            if capacity_1 == goal {
                return Some(BucketStats {
                    moves: 2,
                    goal_bucket: Bucket::Two,
                    other_bucket: capacity_2,
                });
            }
        }
    }

    let mut b = match start_bucket {
        &Bucket::One => (Bucket::One, capacity_1, 0),
        &Bucket::Two => (Bucket::Two, 0, capacity_2),
    };

    let mut moves = 1;
    while b.1 != goal && b.2 != goal {
        match b.0 {
            Bucket::One => {
                let b1_empty = b.1 == 0;
                let b2_full = b.2 == capacity_2;

                if b1_empty {
                    b.1 = capacity_1;
                } else if b2_full {
                    b.2 = 0;
                } else {
                    let move_amt = cmp::min(capacity_2 - b.2, b.1);
                    b.1 -= move_amt;
                    b.2 += move_amt;
                }
            }
            Bucket::Two => {
                let b1_full = b.1 == capacity_1;
                let b2_empty = b.2 == 0;
                if b2_empty {
                    b.2 = capacity_2;
                } else if b1_full {
                    b.1 = 0;
                } else {
                    let move_amt = cmp::min(capacity_1 - b.1, b.2);
                    b.1 += move_amt;
                    b.2 -= move_amt;
                }
            }
        }
        moves += 1;
    }

    let (goal_bucket, other_bucket) = if b.1 == goal {
        (Bucket::One, b.2)
    } else {
        (Bucket::Two, b.1)
    };

    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
}
