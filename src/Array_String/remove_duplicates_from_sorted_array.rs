pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        for j in 1..nums.len() {
            if nums[j] != nums[i - 1] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}
