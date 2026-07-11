use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut ans = 0;
        let mut visited = vec![false; n];

        for u in 0..n {
            if visited[u] {
                continue;
            }

            let mut q = VecDeque::new();

            let (mut count, mut edge) = (1, 0);

            q.push_back(u);
            visited[u] = true;
            while let Some(curr) = q.pop_front() {
                for &v in &graph[curr] {
                    if !visited[v] {
                        count += 1;
                        visited[v] = true;
                        q.push_back(v);
                    }
                    edge += 1;
                }
            }

            if edge == (count - 1) * count {
                ans += 1;
            }
        }
        ans
    }
}
