use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut prequisites = vec![0; n];

        for rel in relations {
            let u = (rel[0] - 1) as usize;
            let v = (rel[1] - 1) as usize;
            prequisites[v] |= 1 << u;
        }

        let full_mask = (1 << n) - 1;

        let mut dp = HashMap::new();
        fn dfs(
            mask: usize,
            n: usize,
            k: usize,
            prequisites: &Vec<i32>,
            full_mask: usize,
            dp: &mut HashMap<usize, i32>,
        ) -> i32 {
            if mask == full_mask {
                return 0;
            }
            if let Some(&res) = dp.get(&mask) {
                return res;
            }

            let mut avail = 0usize;
            for i in 0..n {
                if (mask >> i) & 1 == 0
                    && (mask & prequisites[i] as usize) == prequisites[i] as usize
                {
                    avail |= 1 << i;
                }
            }

            let res = if avail.count_ones() as usize <= k {
                1 + dfs(mask | avail, n, k, prequisites, full_mask, dp)
            } else {
                let mut best = i32::MAX;
                let mut subset = avail;
                while subset > 0 {
                    if subset.count_ones() as usize <= k {
                        best = best.min(1 + dfs(mask | subset, n, k, prequisites, full_mask, dp));
                    }
                    subset = (subset - 1) & avail;
                }
                best
            };
            dp.insert(mask, res);
            res
        }
        dfs(0, n, k, &prequisites, full_mask, &mut dp)
    }
}
