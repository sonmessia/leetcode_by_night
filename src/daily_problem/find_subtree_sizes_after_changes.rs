struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_subtree_sizes(parents: Vec<i32>, s: String) -> Vec<i32> {
        let n = s.len();
        let chars: Vec<u8> = s.into_bytes();
        let mut children: Vec<HashSet<usize>> = vec![HashSet::new(); n];

        for (i, &p) in parents.iter().enumerate() {
            if p >= 0 {
                children[p as usize].insert(i);
            }
        }
        let mut ans = vec![0; n];
        let mut anc: HashMap<u8, usize> = HashMap::new();

        fn dfs(
            u: usize,
            parents: &mut Vec<i32>,
            chars: &Vec<u8>,
            children: &Vec<HashSet<usize>>,
            anc: &mut HashMap<u8, usize>,
            ans: &mut Vec<i32>,
        ) {
            let c = chars[u];
            let prev = anc.insert(c, u);

            ans[u] = 1;
            for &v in &children[u] {
                dfs(v, parents, chars, children, anc, ans);
                let p = parents[v] as usize;
                ans[p] += ans[v];
            }

            if let Some(prev) = prev {
                anc.insert(c, prev);
                parents[u] = prev as i32;
            } else {
                anc.remove(&c);
            }
        }

        dfs(
            0,
            &mut parents.clone(),
            &chars,
            &children,
            &mut anc,
            &mut ans,
        );

        ans
    }
}
