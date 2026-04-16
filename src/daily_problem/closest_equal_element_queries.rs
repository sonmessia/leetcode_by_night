use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut indices_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let n = nums.len();
        let mut ans = vec![-1; queries.len()];

        for i in 0..n {
            indices_map.entry(nums[i]).or_insert(Vec::new()).push(i);
        }

        for indice in indices_map.values() {
            let length = indice.len();

            if length == 1 {
                continue;
            }

            for i in 0..length {
                let curr = indice[i];
                let left = indice[(i - 1 + length) % length];
                let right = indice[(i + 1) % length];
                let left_dist = (curr - left + n) % n;
                let right_dist = (right - curr + n) % n;
                ans[curr] = left_dist.min(right_dist) as i32;
            }
        }

        queries.iter().map(|&q| ans[q as usize]).collect()
    }
}
