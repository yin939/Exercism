pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut flags = vec![true; (upper_bound + 1) as usize];
    let numbers = (0..=upper_bound).collect::<Vec<_>>();
    flags[0] = false;
    flags[1] = false;
    for i in 2..=(upper_bound as f64).sqrt() as usize {
        if flags[i] {
            for j in ((i * i)..=upper_bound as usize).step_by(i) {
                flags[j] = false;
            }
        }
    }

    flags
        .iter()
        .zip(numbers.iter())
        .filter(|(&b, _)| b == true)
        .map(|(_, &v)| v)
        .collect()
}
