struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let set: std::collections::HashSet<char> = word.chars().collect();

        let cnt = set.len() / 8;
        let remainder = set.len() % 8;
        ((cnt * (cnt + 1)) * 4 + (cnt + 1) * remainder) as i32
    }
}
