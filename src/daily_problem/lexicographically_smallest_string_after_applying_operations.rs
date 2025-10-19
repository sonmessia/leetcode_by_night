struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        use std::collections::{HashSet, VecDeque};
        let n = s.len();

        let mut smallest_string = s.clone();

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(s.clone());
        queue.push_back(s.clone());

        while let Some(curr_s) = queue.pop_front() {
            if curr_s < smallest_string {
                smallest_string = curr_s.clone();
            }

            let next_add = apply_add(&curr_s, a);
            if visited.insert(next_add.clone()) {
                queue.push_back(next_add);
            }

            let next_rotate = apply_rotate(&curr_s, b);
            if visited.insert(next_rotate.clone()) {
                queue.push_back(next_rotate);
            }
        }

        fn apply_add(s: &str, a: i32) -> String {
            let mut chars: Vec<char> = s.chars().collect();
            let add_val = a as u8;

            for i in (1..chars.len()).step_by(2) {
                let digit = chars[i].to_digit(10).unwrap() as u8;
                let new_digit = (digit + add_val) % 10;
                chars[i] = char::from_digit(new_digit as u32, 10).unwrap();
            }

            chars.into_iter().collect()
        }

        fn apply_rotate(s: &str, b: i32) -> String {
            let n = s.len();
            let mut result = String::with_capacity(n);
            let rotate_amount = (b as usize) % n;

            result.push_str(&s[n - rotate_amount..]);
            result.push_str(&s[..n - rotate_amount]);

            result
        }

        smallest_string
    }
}
