struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = vec![];
        for time in time_points {
            let parts: Vec<&str> = time.split(':').collect();
            let hour: i32 = parts[0].parse().unwrap();
            let minute: i32 = parts[1].parse().unwrap();
            minutes.push(hour * 60 + minute);
        }

        minutes.sort_unstable();
        let mut min_diff = 1440 + minutes[0] - minutes.last().unwrap();
        for i in 1..minutes.len() {
            let diff = minutes[i] - minutes[i - 1];
            min_diff = min_diff.min(diff);
        }

        min_diff
    }
}
