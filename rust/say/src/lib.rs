pub fn search(n: u64) -> String {
    // 本来想做个全局的 HashMap 或者 Vec。
    // 但是，Rust 不能在全局变量分配堆，所以用 match 匹配也是个好方法，甚至更方便点...我发现。
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        21..=99 => format!("{}-{}", search(n - n % 10), search(n % 10)),
        _ => "".to_string(),
    }
}
// 本来在末尾应该有 "and" 字符需要处理
// 但是测试样例里面没有，似乎简化了。。。就通过了
pub fn encode(n: u64) -> String {
    if n < 100 {
        return search(n);
    }
    let units = vec![
        "hundred",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];
    let mut ans = String::new();
    let mut container: Vec<u64> = Vec::new();
    let mut n = n;
    while n > 0 {
        container.push(n.rem_euclid(1000));
        n = n.div_euclid(1000);
    }
    // 数字取模是逆向存储的，需要转置
    container.reverse();
    // println!("{:?}", container);
    for (index, num) in container.iter().enumerate() {
        if *num == 0 {
            continue;
        }
        // 做数字的单位匹配
        if index < container.len() - 1 {
            ans.push_str(
                format!(
                    "{} {} ",
                    encode_inner_hundred(*num),
                    units[container.len() - index - 1]
                )
                .as_str(),
            );
        } else {
            ans.push_str(format!("{}", encode_inner_hundred(*num)).as_str());
        }
    }

    // 在处理中，3 位数在末尾会留下一个空格符，所以要删除掉
    ans.trim_end().to_string()
}

// 每 3 位数分一段，做好每段 3 位数的格式化
pub fn encode_inner_hundred(n: u64) -> String {
    let mut n = n;
    let mut result: String = String::new();
    while n > 0 {
        if n >= 100 {
            result.push_str(format!("{} hundred ", search(n.div_euclid(100))).as_str());
            n = n.rem_euclid(100);
        } else {
            result.push_str(search(n).as_str());
            break;
        }
    }

    result
}
