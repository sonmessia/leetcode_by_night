struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut prev, mut curr) = (1, 2);
        let n = nums.len();

        while curr < n {
            if nums[curr] == nums[prev] && nums[curr] == nums[prev - 1] {
                curr += 1;
            } else {
                prev += 1;
                nums[prev] = nums[curr];
                curr += 1;
            }
        }
        prev + 1
    }
}
