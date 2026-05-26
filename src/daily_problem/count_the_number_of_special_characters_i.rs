struct Solution;

impl Solution {
    pub fn count_special_characters(word: String) -> i32 {
        word.chars()
            .collect::<std::collections::HashSet<char>>()
            .into_iter()
            .fold(0, |mut acc, c| {
                if c.is_lowercase() {
                    let c = c.to_uppercase().next().unwrap();
                    if word.contains(c) {
                        acc += 1;
                    }
                }
                acc
            })
    }
}
