use std::collections::BTreeSet;
struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut set = BTreeSet::new();

        let helper = |i: usize, j: usize, k: usize| -> i32 {
            let mut sum = 0;

            for t in 0..k {
                sum += grid[i + t][j + t];
            }

            for t in 0..k {
                sum += grid[i + k + t][j + k - t];
            }

            for t in 0..k {
                sum += grid[i + (2 * k) - t][j - t];
            }

            for t in 0..k {
                sum += grid[i + k - t][j - k + t];
            }

            sum
        };

        for i in 0..m {
            for j in 0..n {
                set.insert(grid[i][j]);

                let mut k = 1;
                loop {
                    if i + (2 * k) >= m || j + k >= n || j < k {
                        break;
                    }

                    set.insert(helper(i, j, k));

                    k += 1;
                }
            }
        }

        set.into_iter().rev().take(3).collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::get_biggest_three(vec![
            vec![3, 4, 5, 1, 3],
            vec![3, 3, 4, 2, 3],
            vec![20, 30, 200, 40, 10],
            vec![1, 5, 5, 4, 1],
            vec![4, 3, 2, 2, 5]
        ])
    );
}
