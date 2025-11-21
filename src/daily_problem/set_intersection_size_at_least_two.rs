struct Solution;

impl Solution {
    pub fn intersection_size_at_least_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        let mut res = 0;
        let mut first = -1;
        let mut second = -1;

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];

            if start <= first {
                continue;
            } else if start > first && start <= second {
                res += 1;
                first = second;
                second = end;
            } else {
                res += 2;
                first = end - 1;
                second = end;
            }
        }

        res
    }
}
