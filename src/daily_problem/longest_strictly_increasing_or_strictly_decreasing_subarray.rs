struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        (1..nums.len())
            .fold((1, 1, 1), |(max_len, inc, dec), i| {
                let (inc, dec) = match nums[i].cmp(&nums[i - 1]) {
                    std::cmp::Ordering::Greater => (inc + 1, 1),
                    std::cmp::Ordering::Less => (1, dec + 1),
                    std::cmp::Ordering::Equal => (1, 1),
                };
                (max_len.max(inc).max(dec), inc, dec)
            })
            .0
    }
}
