pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 || size == 1 {
        return if size == 0 { vec![] } else { vec![vec![1]] };
    }

    let size: usize = size as usize;
    let mut rel = vec![vec![0_u32; size]; size];
    // compute the border of left, top, right and bottom.
    let mut l = 0;
    let mut t = 0;
    let mut b = size - 1;
    let mut r = size - 1;
    // keep a default num.
    let mut num = 1_u32;

    while num <= (size * size) as u32 {
        for i in l..=r {
            rel[t][i] = num;
            num += 1;
        }
        t += 1;

        for i in t..=b {
            rel[i][r] = num;
            num += 1;
        }
        r -= 1;

        for i in (l..=r).rev() {
            rel[b][i] = num;
            num += 1;
        }
        b -= 1;

        for i in (t..=b).rev() {
            rel[i][l] = num;
            num += 1;
        }
        l += 1;
    }

    rel
}
