struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let closest_x = x_center.clamp(x1, x2);
        let closest_y = y_center.clamp(y1, y2);
        let distance_x = x_center - closest_x;
        let distance_y = y_center - closest_y;
        distance_x * distance_x + distance_y * distance_y <= radius * radius
    }
}
