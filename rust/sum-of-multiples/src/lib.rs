use std::u32;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // if limit == 1 {
    //     return 0;
    // }
    let mut sum = 0;
    for i in 1..limit {
        for fac in factors {
            if *fac != 0 && i % *fac == 0 {
                sum += i;
                break;
            }
        }
    }
    sum
}
