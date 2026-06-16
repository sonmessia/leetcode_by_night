struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c.is_lowercase() {
                stack.push(c);
            } else if c == '*' {
                stack.pop();
            } else if c == '#' {
                stack = [stack.clone(), stack.clone()].concat();
            } else {
                stack.reverse();
            }
        }
        stack.into_iter().collect()
    }
}
