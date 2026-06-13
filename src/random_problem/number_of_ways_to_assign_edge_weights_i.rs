use std::collections::VecDeque;
struct Solution;

impl Solution {
    fn pow(base: i32, exp: i32) -> i32 {
        let mut result = 1;
        let mut base = base as i64;
        let mut exp = exp as i64;
        const MOD: i64 = 1_000_000_007;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % MOD;
            }
            base = (base * base) % MOD;
            exp /= 2;
        }

        return result as i32;
    }
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; edges.len() + 1];

        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }

        let mut maximum_depth = 0;

        let mut visited = vec![false; graph.len()];
        visited[1] = true;

        let mut current_nodes = VecDeque::from([1]);
        while !current_nodes.is_empty() {
            let len = current_nodes.len();

            for _ in 0..len {
                let node = current_nodes.pop_front().unwrap();

                for &neighbor in &graph[node as usize] {
                    if !visited[neighbor as usize] {
                        visited[neighbor as usize] = true;
                        current_nodes.push_back(neighbor);
                    }
                }
            }

            if !current_nodes.is_empty() {
                maximum_depth += 1;
            }
        }

        Self::pow(2, maximum_depth - 1)
    }
}
