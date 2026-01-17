struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut max_area = 0i64;

        let n = bottom_left.len();
        for i in 0..n {
            for j in i + 1..n {
                if top_right[i][0] <= bottom_left[j][0]
                    || top_right[j][0] <= bottom_left[i][0]
                    || top_right[i][1] <= bottom_left[j][1]
                    || top_right[j][1] <= bottom_left[i][1]
                {
                    continue;
                }
                let w =
                    top_right[i][0].min(top_right[j][0]) - bottom_left[i][0].max(bottom_left[j][0]);
                let h =
                    top_right[i][1].min(top_right[j][1]) - bottom_left[i][1].max(bottom_left[j][1]);
                max_area = max_area.max((w.min(h) as i64).pow(2));
            }
        }

        max_area
    }
}
