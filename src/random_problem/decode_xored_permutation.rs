struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let mut perm = vec![0; n];
        let total_xor = (1..n as i32).fold(0, |acc, x| acc ^ x);
        let mut odd_xor = 0;
        for i in (1..encoded.len()).step_by(2) {
            odd_xor ^= encoded[i];
        }
        perm[0] = total_xor ^ odd_xor;
        for i in 1..n {
            perm[i] = perm[i - 1] ^ encoded[i - 1];
        }
        perm
    }
}
