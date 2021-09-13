use std::collections::HashSet;

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }
    let mut rel = 0;
    let mut books = Vec::from(books);
    books.sort_unstable();
    let mut set_count = [0_u32; 6];

    while !books.is_empty() {
        let s: HashSet<u32> = books.clone().into_iter().collect();
        let len = s.len();
        set_count[len] += 1;
        for val in s {
            let i = books.binary_search(&val).unwrap();
            books.remove(i);
        }
    }

    while set_count[3] > 0 && set_count[5] > 0 {
        set_count[3] -= 1;
        set_count[5] -= 1;
        set_count[4] += 2;
    }
    let price = |size: usize| -> u32 {
        match size {
            1 => 800,
            2 => 1520,
            3 => 2160,
            4 => 2560,
            5 => 3000,
            _ => 0,
        }
    };

    for (i, count) in set_count.iter().enumerate().skip(1) {
        rel += count * price(i);
    }
    rel
}
