struct Solution;

impl Solution {
    pub fn xor_beauty(mut nums: Vec<i32>) -> i32 {
        queries.into_iter().fold(0, |acc, x| acc ^ x)
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}
