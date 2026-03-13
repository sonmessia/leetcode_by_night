struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut ans: i64 = -1;
        let (mut left, mut right) = (0i64, 1_000_000_000_000_000_000);

        while left <= right {
            let mid = left + (right - left) / 2;
            let tmp = worker_times.iter().fold(0, |acc, &x| {
                let work = mid as f64 / x as f64;
                let k = ((-1.0 + (1.0 + work * 8.0).sqrt()) / 2.0).floor() as i64;
                acc + k
            });

            if tmp >= mountain_height as i64 {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        ans
    }
}
