use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();

        for (i, &x) in arr.iter().enumerate() {
            hash.entry(x).or_insert(Vec::new()).push(i);
        }
        let mut visited = vec![false; arr.len()];
        visited[0] = true;

        let mut q = VecDeque::new();
        q.push_back(0);
        let target = arr.len() - 1;

        let mut level = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(pos) = q.pop_front() {
                    if pos == target {
                        return level;
                    }

                    visited[pos] = true;

                    let mut u = Vec::new();
                    if pos > 0 {
                        u.push(pos - 1);
                    }
                    if pos < target {
                        u.push(pos + 1);
                    }

                    if let Some(equal) = hash.get(&arr[pos]) {
                        u.extend(equal);
                    }
                    for next in u {
                        if !visited[next] {
                            if next == target {
                                return level + 1;
                            }
                            visited[next] = true;
                            q.push_back(next);
                        }
                    }
                    hash.remove(&arr[pos]);
                }
            }
            level += 1;
        }
        level
    }
}
