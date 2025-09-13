use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn find_subtree_sizes_after_change(mut parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let chars = s.into_bytes();
        let mut children = vec![HashSet::new(); n];

        let mut ans = vec![0; n];
        let mut anc: HashMap<u8, usize> = HashMap::new();

        for (i, &p) in parent.iter().enumerate() {
            if p >= 0 {
                children[p as usize].insert(i);
            }
        }

        fn dfs(
            u: usize,
            parent: &mut Vec<i32>,
            chars: &Vec<u8>,
            children: &Vec<HashSet<usize>>,
            ans: &mut Vec<i32>,
            anc: &mut HashMap<u8, usize>,
        ) {
            let c = chars[u];
            let prev = anc.insert(c, u);

            ans[u] = 1;
            for &v in &children[u] {
                dfs(v, parent, chars, children, ans, anc);
                let p = parent[v] as usize;
                ans[p] += ans[v];
            }

            if let Some(prev_u) = prev {
                anc.insert(c, prev_u);
                parent[u] = prev_u as i32;
            } else {
                anc.remove(&c);
            }
        }

        dfs(0, &mut parent, &chars, &children, &mut ans, &mut anc);
        ans
    }
}

fn main() {
    let parent = vec![-1, 0, 0, 1, 1, 1];
    let s = "abaabc".to_string();
    let ans = Solution::find_subtree_sizes_after_change(parent, s);
    println!("{:?}", ans);
}
