struct Solution;

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut ans, mut prev) = (0, i32::MIN);
        for &num in &nums {
            let curr = (prev + 1).max(num - k).min(num + k);
            if curr > prev {
                ans += 1;
                prev = curr;
            }
        }
        ans
    }
}
