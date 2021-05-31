use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// 为每个字符分配一个数字，带进算式计算。。。
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // 分离式子
    let mut iter = input.split("==").map(|x| x.trim());

    // 加法部分
    let ops: Vec<_> = iter.next().unwrap().split('+').map(|x| x.trim()).collect();
    // 答案部分
    let result = iter.next().unwrap();

    if ops.iter().any(|o| o.len() > result.len()) {
        return None;
    }

    // 获取所有字母
    let letters: String = ops
        .iter()
        .flat_map(|s| s.chars())
        .chain(result.chars())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // 遍历，计算
    for map in (0..10).permutations(letters.len()).map(|v| {
        letters
            .chars()
            .zip(v.into_iter())
            .collect::<HashMap<_, _>>()
    }) {
        let left: u64 = ops.iter().map(|s| calc_value(&map, s)).sum();
        let right = calc_value(&map, &result);

        if left == right && map.get(&result.chars().next().unwrap()).unwrap() != &0 {
            return Some(map);
        }
    }

    None
}

fn calc_value(map: &HashMap<char, u8>, s: &str) -> u64 {
    s.chars()
        .rev()
        .map(|c| *map.get(&c).unwrap() as u64)
        .enumerate()
        .fold(0, |acc, (e, value)| acc + value * 10_u64.pow(e as u32))
}
