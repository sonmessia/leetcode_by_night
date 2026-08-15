struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut max_length = 0;
        let mut char_count: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        let mut left = 0;

        for right in 0..s.len() {
            let c = s.chars().nth(right).unwrap();
            *char_count.entry(c).or_insert(0) += 1;

            while char_count[&c] > 2 {
                let left_char = s.chars().nth(left).unwrap();
                *char_count.get_mut(&left_char).unwrap() -= 1;
                if char_count[&left_char] == 0 {
                    char_count.remove(&left_char);
                }
                left += 1;
            }

            max_length = max_length.max(right - left + 1);
        }

        max_length as i32
    }
}
