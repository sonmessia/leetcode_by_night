struct Solution;

impl Solution {
    pub fn smallest_missing_multiple_of_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut seen = vec![false; 201];

        for &num in nums.iter() {
            seen[num as usize] = true;
        }

        let mut ans = k;

        while seen[ans as usize] {
            ans += k;
        }

        ans
    }
}
