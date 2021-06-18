#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len_a = first_list.len();
    let len_b = second_list.len();
    match (len_a, len_b) {
        (0, 0) => return Comparison::Equal,
        (0, _) => return Comparison::Sublist,
        (_, 0) => return Comparison::Superlist,
        (_, _) => {
            if len_a < len_b {
                for window in second_list.windows(len_a) {
                    if window[0] == first_list[0] && window == first_list {
                        return Comparison::Sublist;
                    }
                }
            } else if len_a > len_b {
                for window in first_list.windows(len_b) {
                    if window[0] == second_list[0] && window == second_list {
                        return Comparison::Superlist;
                    }
                }
            } else {
                if first_list == second_list {
                    return Comparison::Equal;
                }
            }
        }
    }
    Comparison::Unequal
}
