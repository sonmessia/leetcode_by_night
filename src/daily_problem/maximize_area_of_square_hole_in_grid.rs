struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(
        n: i32,
        m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        h_bars.sort_unstable();
        v_bars.sort_unstable();

        let mut h_max = 1;
        let mut v_max = 1;
        let mut h_curr = 1;
        let mut v_curr = 1;

        for i in 1..h_bars.len() {
            if h_bars[i] == h_bars[i - 1] + 1 {
                h_curr += 1;
            } else {
                h_curr = 1;
            }
            h_max = h_max.max(h_curr);
        }

        for i in 1..v_bars.len() {
            if v_bars[i] == v_bars[i - 1] + 1 {
                v_curr += 1;
            } else {
                v_curr = 1;
            }
            v_max = v_max.max(v_curr);
        }

        let ans = h_max.min(v_max);
        ans * ans
    }
}
