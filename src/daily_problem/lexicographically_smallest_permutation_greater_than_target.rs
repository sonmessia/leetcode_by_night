struct Solution;

impl Solution {
    fn can_form_greater(freq: &[i32; 26], target: &str) -> bool {
        let max_str = Self::get_max_string(freq);
        max_str.as_str() > target
    }

    fn get_min_string(freq: &[i32; 26]) -> String {
        (0..26)
            .flat_map(|i| std::iter::repeat_n((b'a' + i as u8) as char, freq[i] as usize))
            .collect()
    }

    fn get_max_string(freq: &[i32; 26]) -> String {
        (0..26)
            .rev()
            .flat_map(|i| std::iter::repeat_n((b'a' + i as u8) as char, freq[i] as usize))
            .collect()
    }

    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut freq = [0; 26];

        for c in s.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = String::new();

        for (idx, c) in target.chars().enumerate() {
            let target_idx = (c as u8 - b'a') as usize;

            if freq[target_idx] > 0 {
                freq[target_idx] -= 1;
                if Self::can_form_greater(&freq, &target[idx + 1..]) {
                    ans.push(c);
                    continue;
                }
                freq[target_idx] += 1;
            }

            for j in (target_idx + 1)..26 {
                if freq[j] > 0 {
                    freq[j] -= 1;
                    ans.push((b'a' + j as u8) as char);
                    ans.push_str(&Self::get_min_string(&freq));
                    return ans;
                }
            }

            return String::new();
        }

        ans
    }
}
