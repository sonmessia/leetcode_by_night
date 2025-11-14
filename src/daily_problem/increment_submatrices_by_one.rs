struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0; n + 1]; n + 1];

        for query in queries {
            let r1 = query[0] as usize;
            let c1 = query[1] as usize;
            let r2 = query[2] as usize;
            let c2 = query[3] as usize;

            diff[r1][c1] += 1;
            diff[r1][c2 + 1] -= 1;
            diff[r2 + 1][c1] -= 1;
            diff[r2 + 1][c2 + 1] += 1;
        }

        let mut matrix = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                let mut val = diff[i][j];
                if i > 0 {
                    val += matrix[i - 1][j];
                }
                if j > 0 {
                    val += matrix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    val -= matrix[i - 1][j - 1];
                }
                matrix[i][j] = val;
            }
        }
        matrix
    }
}

fn main() {
    let n = 5;
    let queries = vec![vec![1, 1, 3, 3]];
    let result = Solution::range_add_queries(n, queries);
    for row in result {
        println!("{:?}", row);
    }
}
