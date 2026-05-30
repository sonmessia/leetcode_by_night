use std::cmp::max;
use std::collections::BTreeSet;

struct SegTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        SegTree {
            tree: vec![0; 4 * n + 1],
            n,
        }
    }

    fn update(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            self.tree[node] = val;
            return;
        }
        let mid = start + (end - start) / 2;
        if idx <= mid {
            self.update(2 * node, start, mid, idx, val);
        } else {
            self.update(2 * node + 1, mid + 1, end, idx, val);
        }
        self.tree[node] = max(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    fn query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if l > end || r < start {
            return 0;
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = start + (end - start) / 2;
        let mut res = 0;
        if l <= mid {
            res = max(res, self.query(2 * node, start, mid, l, r));
        }
        if r > mid {
            res = max(res, self.query(2 * node + 1, mid + 1, end, l, r));
        }
        res
    }
}
struct Solution;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut results = Vec::new();
        let max_x = queries.iter().map(|q| q[1] as usize).max().unwrap_or(1);

        let mut seg_tree = SegTree::new(max_x);
        let mut obstacles = BTreeSet::from([0usize]);

        for query in queries {
            match query.as_slice() {
                &[1, x] => {
                    let x = x as usize;
                    let &prev = obstacles.range(..x).next_back().unwrap();
                    let next = obstacles.range((x + 1)..).next().copied();

                    seg_tree.update(1, 0, max_x, x, (x - prev) as i32);
                    if let Some(n) = next {
                        seg_tree.update(1, 0, max_x, n, (n - x) as i32);
                    }
                    obstacles.insert(x);
                }
                &[2, x, sz] => {
                    let (x, sz) = (x as usize, sz as i32);
                    let &prev = obstacles.range(..=x).next_back().unwrap();

                    let max_gap_before = seg_tree.query(1, 0, max_x, 0, prev);
                    let max_gap = max(max_gap_before, (x - prev) as i32);

                    results.push(max_gap >= sz);
                }
                _ => {}
            }
        }
        results
    }
}
