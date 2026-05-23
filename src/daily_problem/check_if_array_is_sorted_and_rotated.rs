struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] > nums[(i + 1) % nums.len()] {
                count += 1;
            }
        }
        count <= 1
    }
}
