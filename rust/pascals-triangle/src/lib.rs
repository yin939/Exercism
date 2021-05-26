pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();
        for i in 0..row_count as usize {
            let mut temp = Vec::new();

            for j in 0..=i {
                if j == 0 || j == i {
                    temp.push(1);
                } else if i > 0 && j > 0 {
                    temp.push(triangle[i - 1][j - 1] + triangle[i - 1][j]);
                }
            }
            triangle.push(temp.clone());
        }

        PascalsTriangle { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
