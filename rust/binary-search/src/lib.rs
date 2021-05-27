pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mut left = 0_usize;
    let mut right = array.len() - 1;
    while left <= right {
        let middle = left + ((right - left) >> 1);
        if array[middle] == key {
            return Some(middle);
        } else if array[middle] < key {
            left = middle + 1;
        } else {
            if middle == 0 {
                return None;
            }
            right = middle - 1;
        }
    }

    None
}
