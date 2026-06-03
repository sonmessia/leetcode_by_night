struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let mut ans = i32::MAX;

        let n = land_start_time.len();
        let m = water_start_time.len();

        // Take land ride first
        let mut min_end = i32::MAX;

        for i in 0..n {
            min_end = min_end.min(land_start_time[i] + land_duration[i]);
        }

        for i in 0..m {
            ans = ans.min(water_start_time[i].max(min_end) + water_duration[i]);
        }

        // Take water ride first
        min_end = i32::MAX;

        for i in 0..m {
            min_end = min_end.min(water_start_time[i] + water_duration[i]);
        }

        for i in 0..n {
            ans = ans.min(land_start_time[i].max(min_end) + land_duration[i]);
        }
        ans
    }
}
