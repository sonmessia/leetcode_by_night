struct Solution;

impl Solution {
    fn check(limit_y: f64, squares: &Vec<Vec<i32>>, total_area: f64) -> bool {
        let mut area = 0.0;

        for sq in squares.iter() {
            let y = sq[1] as f64;
            let l = sq[2] as f64;
            if y < limit_y {
                let h = (limit_y - y).min(l);
                area += h * l;
            }
        }

        area >= total_area / 2.0
    }
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let total_area: f64 = squares.iter().map(|sq| sq[2] as f64 * sq[2] as f64).sum();
        let mut max_y: f64 = squares
            .iter()
            .map(|sq| (sq[1] + sq[2]) as f64)
            .fold(0.0, |a, b| a.max(b));

        let mut left = 0.0;
        let mut right = max_y;
        let eps = 1e-5;

        while (right - left).abs() > eps {
            let mid = (left + right) / 2.0;
            if Self::check(mid, &squares, total_area) {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    }
}
