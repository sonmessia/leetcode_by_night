use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn find_subtree_sizes_after_change(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let chars = s.into_bytes();
        let mut children = vec![HashSet::new(); n];

        for (i, &p) in parent.iter().enumerate() {
            if p >= 0 {
                children[p as usize].insert(i);
            }
        }
        let mut ans = vec![0; n];
        let mut anc: HashMap<u8, usize> = HashMap::new();

        fn dfs() {}

        ans
    }
}
