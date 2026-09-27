struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = Vec::new();
        let mut current = String::new();

        for c in s.chars() {
            if c == '(' {
                stack.push(current);
                current = String::new();
            } else if c == ')' {
                let mut prev = stack.pop().unwrap();
                current = current.chars().rev().collect::<String>();
                prev.push_str(&current);
                current = prev;
            } else {
                current.push(c);
            }
        }

        current
    }
}
