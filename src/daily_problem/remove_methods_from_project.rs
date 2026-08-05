struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for invocation in invocations {
            graph[invocation[0] as usize].push(invocation[1]);
        }

        let mut checked = vec![false; n];
        checked[k as usize] = true;

        let mut ans = vec![];

        let mut q = std::collections::VecDeque::new();
        q.push_back(k);

        while let Some(node) = q.pop_front() {
            for &neighbor in &graph[node as usize] {
                if !checked[neighbor as usize] {
                    checked[neighbor as usize] = true;
                    q.push_back(neighbor);
                }
            }
        }

        println!("Checked: {:?}", checked);

        for i in 0..n {
            if !checked[i] {
                for &u in graph[i].iter() {
                    if checked[u as usize] {
                        return (1..=n as i32).collect();
                    }
                }
                ans.push(i as i32);
            }
        }

        ans
    }
}
