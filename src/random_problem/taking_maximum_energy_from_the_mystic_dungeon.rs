struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let k = k as usize;
        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            if i + k >= n {
                dp[i] = energy[i];
            } else {
                dp[i] += energy[i] + dp[i + k];
            }
        }
        let mut ans = i32::MIN;
        for i in 0..n {
            ans = ans.max(dp[i]);
        }
        ans
    }
}
