struct Solution;

impl Solution {
    pub fn winning_player(x: i32, y: i32) -> String {
        let mut cnt = 0;
        let (mut x, mut y) = (x, y);
        while x > 0 && y > 3 {
            x -= 1;
            y -= 4;
            cnt += 1;
        }
        if cnt % 2 == 1 {
            "Alice".to_owned()
        } else {
            "Bob".to_owned()
        }
    }
}
