struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let n = n as i64;

        let mut batteries = batteries
            .into_iter()
            .map(|x| x as i64)
            .collect::<Vec<i64>>();

        let sum = batteries.iter().sum::<i64>();

        let (mut left, mut right, mut ans) = (0, sum / n, 0);

        while left <= right {
            let mid = left + (right - left) / 2;

            let total = batteries.iter().map(|&x| x.min(mid)).sum::<i64>();

            if total >= mid * n {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}
