struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut count = 0;
        let mut left = 0;
        let mut right = 0;
        let mut char_count = [0; 3];

        while right < s.len() {
            char_count[(s.as_bytes()[right] - b'a') as usize] += 1;

            while char_count.iter().all(|&x| x > 0) {
                count += s.len() - right;
                char_count[(s.as_bytes()[left] - b'a') as usize] -= 1;
                left += 1;
            }

            right += 1;
        }

        count as i32
    }
}
