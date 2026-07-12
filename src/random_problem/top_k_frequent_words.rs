struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for word in words {
            *map.entry(word).or_insert(0) += 1;
        }
        let mut vec: Vec<(String, i32)> = map.into_iter().collect();
        vec.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        vec.into_iter().take(k as usize).map(|x| x.0).collect()
    }
}
