struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 1 && chars[0] == '1' {
            return true;
        }
        for i in 0..(chars.len() - 1) {
            if chars[i] == chars[i + 1] && chars[i] == '1' {
                return true;
            }
        }

        false
    }
}
