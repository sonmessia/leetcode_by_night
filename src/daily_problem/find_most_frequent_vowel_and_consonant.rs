use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut max_vowels = 0;
        let mut max_consonants = 0;
        let vowels = "aeiou".chars().collect::<Vec<char>>();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for (c, count) in map {
            if vowels.contains(&c) {
                max_vowels = max_vowels.max(count);
            } else {
                max_consonants = max_consonants.max(count);
            }
        }

        max_consonants + max_vowels
    }
}
