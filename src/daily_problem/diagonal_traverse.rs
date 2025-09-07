struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (mat.len(), mat[0].len());
        let mut ans: Vec<i32> = vec![0; m * n];

        let (mut i, mut j) = (0, 0);
        for idx in 0..(m * n) {
            ans[idx] = mat[i][j];
            if (i + j) % 2 == 0 {
                if j == m - 1 {
                    i += 1;
                } else if i == 0 {
                    j += 1;
                } else {
                    i -= 1;
                    j += 1;
                }
            }
            if i == n - 1 {
                j += 1;
            } else if j == 0 {
                i += 1;
            } else {
                i += 1;
                j -= 1;
            }
        }
        ans
    }
}

fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = Solution::find_diagonal_order(mat);
    println!("{:?}", result);
}
