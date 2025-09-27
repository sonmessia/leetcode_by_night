struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut max_area: f64 = 0.0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let area = Self::triangle_area(&points[i], &points[j], &points[k]);
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }
    fn triangle_area(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> f64 {
        let x1 = p1[0] as f64;
        let y1 = p1[1] as f64;
        let x2 = p2[0] as f64;
        let y2 = p2[1] as f64;
        let x3 = p3[0] as f64;
        let y3 = p3[1] as f64;
        ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs()) / 2.0
    }
}
