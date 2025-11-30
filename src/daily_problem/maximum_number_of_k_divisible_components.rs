struct Solution;

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        fn dfs(
            node: usize,
            parent: usize,
            adj: &Vec<Vec<usize>>,
            values: &Vec<i32>,
            k: i32,
            count: &mut i32,
        ) -> i32 {
            let mut sum = 0;

            for &neighbor in &adj[node] {
                if neighbor != parent {
                    sum += dfs(neighbor, node, adj, values, k, count);
                    sum %= k;
                }
            }

            sum += values[node];
            sum %= k;

            if sum == 0 {
                *count += 1;
            }
            
            sum
        }

        let mut adj = vec![vec![]; n as usize];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut component_count = 0;
        dfs(0, usize::MAX, &adj, &values, k, &mut component_count);
        return component_count;
    }
}
