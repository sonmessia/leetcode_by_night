struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let (mut cnt, n) = (0, nums.len());

        let k = k as usize;
        if k == 1 {
            return true;
        }
        for i in 1..n - k {
            if nums[i] > nums[i - 1] && nums[i + k] > nums[i + k - 1] {
                cnt += 1;
            } else {
                cnt = 0;
            }
            if cnt == k - 1 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let k = 2;
    let result = Solution::has_increasing_subarrays(nums, k);
    println!("{}", result); // Output: true
}
