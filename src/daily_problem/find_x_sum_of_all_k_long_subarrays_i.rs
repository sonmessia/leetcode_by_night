use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = nums.len();
        let k_usize = k as usize;
        let x_usize = x as usize;
        let mut ans = vec![0; n - k_usize + 1];
        for i in 0..ans.len() {
            let mut freq: HashMap<i32, i32> = HashMap::new();
            for j in 0..k_usize {
                *freq.entry(nums[i + j]).or_insert(0) += 1;
            }

            let mut parr: Vec<(i32, i32)> =
                freq.into_iter().map(|(val, count)| (count, val)).collect();

            parr.sort_by(|a, b| {
                if a.0 != b.0 {
                    b.0.cmp(&a.0)
                } else {
                    b.1.cmp(&a.1)
                }
            });

            let mut sum = 0;
            for (count, value) in parr.iter().take(x_usize) {
                sum += count * value;
            }
            ans[i] = sum;
        }

        ans
    }
}
