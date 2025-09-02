struct Solution;

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let area_a = (ax2 - ax1) * (ay2 - ay1);
        let area_b = (bx2 - bx1) * (by2 - by1);
        let overlap_width = (ax2.min(bx2) - ax1.max(bx1)).max(0);
        let overlap_height = (ay2.min(by2) - ay1.max(by1)).max(0);
        let overlap_area = overlap_width * overlap_height;
        area_a + area_b - overlap_area
    }
}

fn main() {
    let ax1 = -3;
    let ay1 = 0;
    let ax2 = 3;
    let ay2 = 4;
    let bx1 = 0;
    let by1 = -1;
    let bx2 = 9;
    let by2 = 2;
    let result = Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
    println!("Total area covered by the two rectangles: {}", result); // Output: 45
}
