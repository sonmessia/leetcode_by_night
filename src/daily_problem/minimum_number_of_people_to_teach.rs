use std::{
    collections::{HashMap, HashSet},
    usize,
};
struct Solution;

impl Solution {
    pub fn minimum_teaching(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut ans = i32::MAX;

        let mut lang_map: Vec<HashSet<i32>> = languages
            .iter()
            .map(|langs| langs.iter().cloned().collect::<HashSet<i32>>())
            .collect();

        let mut need_to_learn = HashSet::new();

        for f in &friendships {
            let u = (f[0] - 1) as usize;
            let v = (f[1] - 1) as usize;

            if lang_map[u].is_disjoint(&lang_map[v]) {
                need_to_learn.insert(u);
                need_to_learn.insert(v);
            }
        }

        if need_to_learn.is_empty() {
            return 0;
        }

        for lang in 1..=n {
            let mut cnt = 0;
            for &user in &need_to_learn {
                if !lang_map[user].contains(&lang) {
                    cnt += 1;
                }
            }
            ans = ans.min(cnt);
        }
        ans
    }
}
