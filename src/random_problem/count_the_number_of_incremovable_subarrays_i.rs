struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();

        let (mut left, mut right) = (0, n - 1);

        while left + 1 < n && nums[left] < nums[left + 1] {
            left += 1;
        }

        if left == n - 1 {
            return (n * (n + 1) / 2) as i32;
        }

        while right > 0 && nums[right - 1] < nums[right] {
            right -= 1;
        }

        ans += left + 1;
        ans += n - right + 1;

        let (mut i, mut j) = (0, right);
        while i <= left {
            while j < n && nums[i] >= nums[j] {
                j += 1;
            }
            ans += n - j;
            i += 1;
        }
        ans as i32
    }
}

fn main() {
    let nums = vec![1, 4, 2, 3];
    let result = Solution::incremovable_subarray_count(nums);
    println!("{}", result); // Output: 7
}
