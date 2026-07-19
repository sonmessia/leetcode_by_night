struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut stack = Vec::new();
        let mut seen = [false; 26];
        let mut last_occurrence = [0; 26];

        for (i, c) in s.chars().enumerate() {
            last_occurrence[(c as u8 - b'a') as usize] = i;
        }

        for (i, c) in s.chars().enumerate() {
            let index = (c as u8 - b'a') as usize;
            if seen[index] {
                continue;
            }

            while let Some(&last) = stack.last() {
                let last_index = (last as u8 - b'a') as usize;
                if c < last && i < last_occurrence[last_index] {
                    seen[last_index] = false;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(c);
            seen[index] = true;
        }

        stack.into_iter().collect()
    }
}
