struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let reverse = n.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap();
        (n - reverse).abs()
    }
}
