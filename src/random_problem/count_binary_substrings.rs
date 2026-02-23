struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut count = 0;
        let mut prev_run_length = 0;
        let mut curr_run_length = 1;

        for i in 1..s.len() {
            if s.as_bytes()[i] == s.as_bytes()[i - 1] {
                curr_run_length += 1;
            } else {
                count += std::cmp::min(prev_run_length, curr_run_length);
                prev_run_length = curr_run_length;
                curr_run_length = 1;
            }
        }
        count + std::cmp::min(prev_run_length, curr_run_length)
    }
}
