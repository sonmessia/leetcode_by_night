struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut total_time = 0;
        let mut max_time_in_group = 0;
        let mut prev_char: Option<char> = None;

        for (i, c) in colors.chars().enumerate() {
            if Some(c) == prev_char {
                total_time += needed_time[i].min(max_time_in_group);
                max_time_in_group = max_time_in_group.max(needed_time[i]);
            } else {
                max_time_in_group = needed_time[i];
            }
            prev_char = Some(c);
        }

        total_time
    }
}
