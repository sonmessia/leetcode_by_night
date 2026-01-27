struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut graph = vec![vec![]; n as usize];

        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let cost = edge[2];
            graph[u].push((v, cost));
            graph[v].push((u, 2 * cost));
        }

        let mut d = vec![i32::MAX; n as usize];
        d[0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));

        while let Some(Reverse((cost, u))) = heap.pop() {
            if cost > d[u as usize] {
                continue;
            }

            for next in graph[u as usize].iter() {
                let v = next.0;
                let edge_cost = next.1;
                let new_cost = cost + edge_cost;

                if new_cost < d[v as usize] {
                    d[v as usize] = new_cost;
                    heap.push(Reverse((new_cost, v)));
                }
            }
        }

        return if d[(n - 1) as usize] != i32::MAX {
            d[(n - 1) as usize]
        } else {
            -1
        };
    }
}
