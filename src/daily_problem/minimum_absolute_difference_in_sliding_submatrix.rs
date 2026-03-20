use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let k_usize = k as usize;

        let rows = m - k_usize + 1;
        let cols = n - k_usize + 1;

        let mut ans = vec![vec![0; cols]; rows];

        if k_usize == 1 {
            return ans;
        }

        for i in 0..rows {
            for j in 0..cols {
                let mut tmp: Vec<i32> = Vec::with_capacity(k_usize * k_usize);

                for k in i..(i + k_usize) {
                    for l in j..(j + k_usize) {
                        tmp.push(grid[k][l]);
                    }
                }

                tmp.sort_unstable();

                let mut curr_diff = i32::MAX;

                for w in tmp.windows(2) {
                    if w[0] != w[1] {
                        curr_diff = curr_diff.min((w[0] - w[1]).abs());
                    }
                }

                if curr_diff != i32::MAX {
                    ans[i][j] = curr_diff;
                }
            }
        }
        //
        // if k == 1 {
        //     return vec![vec![0; n]; m];
        // }
        // let mut ans = vec![vec![]];
        //
        // for i in 0..m - k as usize + 1 {
        //     for j in 0..n - k as usize + 1 {
        //         let mut dp = BTreeSet::new();
        //         let mut curr_diff = i32::MAX;
        //
        //         for x in i..i + k as usize {
        //             for y in j..j + k as usize {
        //                 dp.insert(grid[x][y]);
        //             }
        //         }
        //
        //         if dp.len() == 1 {
        //             ans[i].push(0);
        //             continue;
        //         }
        //
        //         let mut dp_1 = vec![];
        //
        //         for i in dp {
        //             dp_1.push(i);
        //         }
        //
        //         for i in 0..dp_1.len() {
        //             for j in i + 1..dp_1.len() {
        //                 curr_diff = curr_diff.min((dp_1[i] - dp_1[j]).abs());
        //             }
        //         }
        //
        //         ans[i].push(curr_diff);
        //     }
        //
        //     if i != m - k as usize {
        //         ans.push(vec![]);
        //     }
        // }
        ans
    }
}
