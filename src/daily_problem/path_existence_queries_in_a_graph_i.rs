struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut idx = vec![0; n];
        let mut current_idx = 0;

        for i in 1..n {
            if (nums[i] - nums[i - 1]).abs() > max_diff {
                current_idx += 1;
            }
            idx[i] = current_idx;
        }

        queries
            .iter()
            .map(|query| {
                let u = query[0] as usize;
                let v = query[1] as usize;
                idx[u] == idx[v]
            })
            .collect()
    }
}
