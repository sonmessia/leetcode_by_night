struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut glasses = vec![vec![0.0; 102]; 102];
        glasses[0][0] = poured as f64;
        for i in 0..query_row as usize {
            for j in 0..=i {
                if glasses[i][j] > 1.0 {
                    let excess = (glasses[i][j] - 1.0) / 2.0;
                    glasses[i + 1][j] += excess;
                    glasses[i + 1][j + 1] += excess;
                    glasses[i][j] = 1.0;
                }
            }
        }
        glasses[query_row as usize][query_glass as usize].min(1.0)
    }
}
