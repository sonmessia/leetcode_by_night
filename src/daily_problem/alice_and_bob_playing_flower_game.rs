struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        (n as i64 * m as i64) / 2
    }
}

fn main() {
    let result = Solution::flower_game(3, 4);
    println!("The result is: {}", result);
}
