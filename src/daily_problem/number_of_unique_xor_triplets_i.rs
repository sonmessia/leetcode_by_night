struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n <= 2 {
            return n as i32;
        }

        let log = n.ilog2() + 1;

        1 << log
    }
}
