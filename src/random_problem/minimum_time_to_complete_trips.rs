struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut left: i64 = 1;
        let mut right: i64 = 1e14 as i64;
        let mut result: i64 = right;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mut completed_trips: i64 = 0;

            for &trip in &time {
                completed_trips += mid / trip as i64;
            }

            if completed_trips >= total_trips as i64 {
                result = result.min(mid);
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        result
    }
}
