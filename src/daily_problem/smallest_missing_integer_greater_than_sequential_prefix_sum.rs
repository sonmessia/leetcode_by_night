struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut visited = vec![false; 51];
        let mut sum = nums[0];
        for &num in nums.iter() {
            visited[num as usize] = true;
        }

        let mut i = 1;
        while i < n && nums[i] == nums[i - 1] + 1 {
            sum += nums[i];
            i += 1;
        }

        while sum <= 50 && visited[sum as usize] {
            sum += 1;
        }

        sum
    }
}
