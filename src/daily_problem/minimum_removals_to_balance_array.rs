struct Solution;

impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as i64);
        if n == 1 {
            return 0;
        }

        nums.sort_unstable();
        let mut max_removals = 0;
        let mut r = 0;

        for l in 0..n {
            while r < n && nums[r] as i64 <= (nums[l] as i64 * k) {
                r += 1;
            }
            max_removals = max_removals.max(r - l);
        }

        (n - max_removals) as i32
    }
}
