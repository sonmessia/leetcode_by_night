struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut stack = Vec::new();
        let mut count = 0;
        let mut start = 0;
        for (i, c) in s.char_indices() {
            count += if c == '1' { 1 } else { -1 };
            if count == 0 {
                stack.push(format!(
                    "1{}0",
                    Self::make_largest_special(s[start + 1..i].to_string())
                ));
                start = i + 1;
            }
        }
        stack.sort_unstable_by(|a, b| b.cmp(a));
        stack.concat()
    }
}
