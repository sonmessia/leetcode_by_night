use std::{
    collections::{HashMap, VecDeque},
    sync::LazyLock,
};

struct Solution;
const MAXN: usize = 1_000_000;
static FACTOR: LazyLock<Vec<Vec<usize>>> = LazyLock::new(|| {
    let mut f: Vec<Vec<usize>> = vec![vec![]; MAXN];
    for i in 2..MAXN {
        if f[i].is_empty() {
            for j in (i..MAXN).step_by(i) {
                f[j].push(i);
            }
        }
    }
    f
});

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            if FACTOR[x as usize].len() == 1 {
                graph.entry(x).or_default().push(i);
            }
        }

        let mut ans = 0;
        let mut visited = vec![false; n];
        let mut q = VecDeque::new();
        q.push_back(n - 1);
        visited[n - 1] = true;

        loop {
            let m = q.len();
            for _ in 0..m {
                let i = q.pop_front().unwrap();
                if i == 0 {
                    return ans;
                }
                if i > 0 && !visited[i - 1] {
                    visited[i - 1] = true;
                    q.push_back(i - 1);
                }
                if i < n - 1 && !visited[i + 1] {
                    visited[i + 1] = true;
                    q.push_back(i + 1);
                }
                for &p in &FACTOR[nums[i] as usize] {
                    if let Some(v) = graph.get_mut(&(p as i32)) {
                        for &j in v.iter() {
                            if !visited[j] {
                                visited[j] = true;
                                q.push_back(j);
                            }
                        }
                        v.clear();
                    }
                }
            }
            ans += 1;
        }
    }
}
