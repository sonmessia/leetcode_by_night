struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut sum, mut spaces) = (0i32, 0i32);
        for m in moves.as_bytes() {
            match m {
                0x4c => sum += 1,
                0x52 => sum -= 1,
                _ => spaces += 1,
            };
        };
        return sum.abs() + spaces
    }
}

