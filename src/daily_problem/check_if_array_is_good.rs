struct Solution;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut freq = vec![0; n];
        for num in nums {
            if num >= n as i32 || (num < n as i32 - 1 && freq[num as usize] > 0) || (num == n as i32 - 1 && freq[num as usize] > 1) {
                return false;
            }

            freq[num as usize] += 1;
        }
       true
    }
}
