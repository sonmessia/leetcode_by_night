use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();
        let (mut l, mut r) = (i32::MAX, 0);

        let mut graph = vec![vec![]; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            if online[u] && online[v] {
                graph[u].push((v, w as i64));
                l = l.min(w);
                r = r.max(w);
            }
        }

        // for g in graph.iter() {
        //     println!("{:?}", g);
        // }

        let check = |mid| -> bool {
            fn dfs(u: usize, graph: &Vec<Vec<(usize, i64)>>, memo: &mut Vec<i64>, mid: i32) -> i64 {
                if u == graph.len() - 1 {
                    return 0;
                }

                if memo[u] != -1 {
                    return memo[u];
                }

                let mut res = i64::MAX / 2;
                for &(v, w) in &graph[u] {
                    if w >= mid as i64 {
                        res = res.min(dfs(v, graph, memo, mid) + w);
                    }
                }

                memo[u] = res;
                res
            }

            let mut memo = vec![-1i64; n];
            dfs(0, &graph, &mut memo, mid) <= k
        };

        if !check(l) {
            return -1;
        }

        let mut ans = 0;
        while l <= r {
            let mid = l + (r - l) / 2;
            if check(mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        ans
    }
}
