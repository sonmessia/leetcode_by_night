struct Solution;

impl Solution {
    
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut ans = vec![0; nums.len()];
        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            map.entry(num).or_insert(vec![]).push(i as i64);
        }
        for indices in map.values() {
            let mut prefix_sum = vec![0; indices.len() + 1];
            for i in 0..indices.len() {
                prefix_sum[i + 1] = prefix_sum[i] + indices[i];
            }
            for i in 0..indices.len() {
                let idx = indices[i] as usize;
                ans[idx] = (i as i64 * indices[i] - prefix_sum[i]) + (prefix_sum[indices.len()] - prefix_sum[i + 1] - (indices.len() as i64 - i as i64 - 1) * indices[i]);
            }
        }
        ans
    }
}
