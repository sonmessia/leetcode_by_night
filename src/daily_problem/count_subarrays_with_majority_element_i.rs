struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;
        let mut target_count = 0;

        for (idx, num) in nums.iter().enumerate() {
            if *num == target {
                target_count += 1;
            }

            let mut count = target_count;
            for j in 0..=idx {
                if count * 2 > idx - j + 1 {
                    ans += 1;
                }
                if nums[j] == target {
                    count -= 1;
                }
            }
        }
        ans
    }
}
