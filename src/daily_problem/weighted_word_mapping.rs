struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .iter()
            .map(|w| {
                let sum: i32 = w
                    .as_bytes()
                    .iter()
                    .map(|&ch| weights[(ch - b'a') as usize])
                    .sum();
                let index: i32 = sum % 26;
                (b'z' - index as u8) as char
            })
            .collect()
    }
}
