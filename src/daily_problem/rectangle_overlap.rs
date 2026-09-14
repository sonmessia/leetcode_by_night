struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (x1, y1, x2, y2) = (rec1[0], rec1[1], rec1[2], rec1[3]);
        let (x3, y3, x4, y4) = (rec2[0], rec2[1], rec2[2], rec2[3]);

        if x2 <= x3 || x4 <= x1 {
            return false;
        }

        // Check if one rectangle is above the other
        if y2 <= y3 || y4 <= y1 {
            return false;
        }

        true
    }
}
