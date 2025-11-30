struct Solution;

impl Solution {
    pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let total_sum: i32 = nums.iter().sum();
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut degree = vec![0; n];

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
            degree[u] += 1;
            degree[v] += 1;
        }

        // Try from smallest target_sum (most components) to largest
        // We want maximum edges deleted = maximum components - 1
        for num_components in (2..=n as i32).rev() {
            if total_sum % num_components != 0 {
                continue;
            }
            let target_sum = total_sum / num_components;

            if Self::can_split(&nums, &graph, &degree, target_sum) {
                return num_components - 1;
            }
        }

        0
    }

    fn can_split(nums: &[i32], graph: &[Vec<usize>], degree: &[i32], target_sum: i32) -> bool {
        use std::collections::VecDeque;

        let n = nums.len();
        let mut current_sum: Vec<i32> = nums.to_vec();
        let mut current_degree = degree.to_vec();
        let mut queue: VecDeque<usize> = VecDeque::new();

        for (i, &deg) in current_degree.iter().enumerate() {
            if deg == 1 {
                queue.push_back(i);
            }
        }

        let mut processed = 0;

        while let Some(node) = queue.pop_front() {
            processed += 1;

            if current_sum[node] > target_sum {
                return false;
            }

            // If sum equals target, this component is complete - don't propagate
            let propagate = current_sum[node] != target_sum;

            for &neighbor in &graph[node] {
                if current_degree[neighbor] == 0 {
                    continue;
                }

                if propagate {
                    current_sum[neighbor] += current_sum[node];
                }

                current_degree[neighbor] -= 1;

                if current_degree[neighbor] == 1 {
                    queue.push_back(neighbor);
                }
            }
        }

        // All nodes should be processed
        processed == n
    }
}

fn main() {
    let nums = vec![6, 2, 2, 2, 6];
    let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];

    let result = Solution::component_value(nums, edges);
    println!("Maximum number of edges that can be removed: {}", result);
}
