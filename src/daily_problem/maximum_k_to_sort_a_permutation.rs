struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut encounter_first = false;

        if nums[0] != 0 {
            return 0;
        }

        for (idx, num) in nums.into_iter().enumerate() {
            if !encounter_first && num != idx as i32 {
                ans = num;
                encounter_first = !encounter_first;
            }

            if num != idx as i32 {
                ans &= num;
            }
        }

        ans
    }
}
