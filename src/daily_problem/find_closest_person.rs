struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match (z-x).abs().cmp(&(z-y).abs()) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => 1,
            _ => 2,
        }
    }
}

fn main() {
    let x = 1;
    let y = 2;
    let z = 3;
    let result = Solution::find_closest(x, y, z);
    println!("The closest person to z is: {}", result);
}