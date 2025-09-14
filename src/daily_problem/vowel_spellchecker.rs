use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut exact_match = HashSet::new();
        let mut case_insensitive_map = HashMap::new();
        let mut vowel_error_map = HashMap::new();
        for word in &wordlist {
            exact_match.insert(word.clone());
            let lower_word = word.to_lowercase();
            case_insensitive_map
                .entry(lower_word.clone())
                .or_insert(word.clone());
            let devoweled_word = Self::devowel(&lower_word);
            vowel_error_map
                .entry(devoweled_word)
                .or_insert(word.clone());
        }
        let mut result = Vec::new();
        for query in &queries {
            if exact_match.contains(query) {
                result.push(query.clone());
                continue;
            }
            let lower_query = query.to_lowercase();
            if let Some(matched) = case_insensitive_map.get(&lower_query) {
                result.push(matched.clone());
                continue;
            }
            let devoweled_query = Self::devowel(&lower_query);
            if let Some(matched) = vowel_error_map.get(&devoweled_query) {
                result.push(matched.clone());
                continue;
            }
            result.push(String::new());
        }
        result
    }
    fn devowel(word: &str) -> String {
        word.chars()
            .map(|c| if "aeiou".contains(c) { '*' } else { c })
            .collect()
    }
}

fn main() {
    let wordlist = vec![
        "KiTe".to_string(),
        "kite".to_string(),
        "hare".to_string(),
        "Hare".to_string(),
    ];
    let queries = vec![
        "kite".to_string(),
        "Kite".to_string(),
        "KiTe".to_string(),
        "Hare".to_string(),
        "HARE".to_string(),
        "Hear".to_string(),
        "hear".to_string(),
        "keti".to_string(),
        "keet".to_string(),
        "keto".to_string(),
    ];
    let result = Solution::spellchecker(wordlist, queries);
    for res in result {
        println!("{}", res);
    }
}
