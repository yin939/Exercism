use std::u32;

pub fn is_armstrong_number(num: u32) -> bool {
    let length = num.to_string().len();
    let sum = num
        .to_string()
        .split("")
        .map(|x| x.parse::<u32>().unwrap_or_default().pow(length as u32))
        .sum();

    num == sum
}
