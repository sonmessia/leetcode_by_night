use std::collections::VecDeque;
struct Solution;

impl Solution {
    pub fn lca(
        mut u: usize,
        mut v: usize,
        log: usize,
        depth: &Vec<usize>,
        lca: &Vec<Vec<usize>>,
    ) -> usize {
        if depth[u] < depth[v] {
            (u, v) = (v, u);
        }

        let diff = depth[u] - depth[v];

        for i in 0..log {
            if (diff & (1 << i)) != 0 {
                u = lca[u][i];
            }
        }

        if u == v {
            return u;
        }

        for i in (0..log).rev() {
            if lca[u][i] != lca[v][i] {
                u = lca[u][i];
                v = lca[v][i];
            }
        }

        lca[u][0]
    }

    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const LOG: usize = 18;
        const MOD: i32 = 1_000_000_007;
        let mut graph = vec![vec![]; edges.len() + 2];
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }

        let mut lca = vec![vec![0; LOG]; edges.len() + 2];
        let mut depth = vec![0; edges.len() + 2];
        let mut queue = VecDeque::new();

        queue.push_back((1, 0));
        while let Some((node, parent)) = queue.pop_front() {
            for &neighbor in &graph[node as usize] {
                if neighbor != parent {
                    depth[neighbor as usize] = depth[node as usize] + 1;
                    lca[neighbor as usize][0] = node as usize;
                    queue.push_back((neighbor, node));
                }
            }
        }

        for j in 1..LOG {
            for i in 1..=edges.len() + 1 {
                lca[i][j] = lca[lca[i][j - 1]][j - 1];
            }
        }

        let mut pow_cache = vec![1; edges.len() + 2];
        pow_cache[1] = 2;

        for i in 2..=edges.len() {
            pow_cache[i] = (pow_cache[i - 1] << 1) % MOD;
        }

        queries
            .iter()
            .map(|query| {
                let (u, v) = (query[0] as usize, query[1] as usize);
                if u == v {
                    0
                } else {
                    let acestors = Self::lca(u, v, LOG, &depth, &lca);
                    let dist = depth[u] + depth[v] - 2 * depth[acestors];
                    pow_cache[dist - 1]
                }
            })
            .collect()
    }
}
