struct Solution;

impl Solution {
    pub fn check_on_ones(s: String) -> bool {
        !s.contains("01")
    }
}
