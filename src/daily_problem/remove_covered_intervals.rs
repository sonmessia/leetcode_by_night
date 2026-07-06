struct Solution;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let mut count = 0;
        let mut prev_end = 0;
        for interval in intervals {
            if interval[1] > prev_end {
                count += 1;
                prev_end = interval[1];
            }
        }
        count
    }
}
