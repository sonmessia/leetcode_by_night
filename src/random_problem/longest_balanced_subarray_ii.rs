use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
struct LazyTag {
    add: i32,
}

impl LazyTag {
    fn new() -> Self {
        Self { add: 0 }
    }
    fn is_empty(&self) -> bool {
        self.add == 0
    }
    fn combine(&mut self, other: &LazyTag) {
        self.add += other.add;
    }
    fn clear(&mut self) {
        self.add = 0;
    }
}

#[derive(Debug, Clone)]
struct Node {
    min_val: i32,
    max_val: i32,
    lazy: LazyTag,
}

impl Node {
    fn new() -> Self {
        Node {
            min_val: 0,
            max_val: 0,
            lazy: LazyTag::new(),
        }
    }
}

struct SegmentTree {
    n: usize,
    tree: Vec<Node>,
}

impl SegmentTree {
    fn new(data: &[i32]) -> Self {
        let n = data.len();
        let mut tree = vec![Node::new(); 4 * n + 5];
        let mut seg = SegmentTree { n, tree };
        seg.build(data, 1, n, 1);
        seg
    }

    fn build(&mut self, data: &[i32], l: usize, r: usize, idx: usize) {
        if l == r {
            self.tree[idx].min_val = data[l - 1];
            self.tree[idx].max_val = data[l - 1];
            return;
        }
        let mid = (l + r) / 2;
        self.build(data, l, mid, idx * 2);
        self.build(data, mid + 1, r, idx * 2 + 1);
        self.push_up(idx);
    }

    fn push_up(&mut self, idx: usize) {
        self.tree[idx].min_val = min(self.tree[idx * 2].min_val, self.tree[idx * 2 + 1].min_val);
        self.tree[idx].max_val = max(self.tree[idx * 2].max_val, self.tree[idx * 2 + 1].max_val);
    }

    fn apply(&mut self, idx: usize, tag: &LazyTag) {
        self.tree[idx].min_val += tag.add;
        self.tree[idx].max_val += tag.add;
        self.tree[idx].lazy.combine(tag);
    }

    fn push_down(&mut self, idx: usize) {
        if !self.tree[idx].lazy.is_empty() {
            let tag = self.tree[idx].lazy;
            self.apply(idx * 2, &tag);
            self.apply(idx * 2 + 1, &tag);
            self.tree[idx].lazy.clear();
        }
    }

    fn range_add(&mut self, l: usize, r: usize, val: i32) {
        if l > r || l > self.n || r < 1 {
            return;
        }
        let valid_l = max(1, l);
        let valid_r = min(self.n, r);
        let tag = LazyTag { add: val };
        self._update(valid_l, valid_r, &tag, 1, self.n, 1);
    }

    fn _update(&mut self, ql: usize, qr: usize, tag: &LazyTag, l: usize, r: usize, idx: usize) {
        if ql <= l && r <= qr {
            self.apply(idx, tag);
            return;
        }
        self.push_down(idx);
        let mid = (l + r) / 2;
        if ql <= mid {
            self._update(ql, qr, tag, l, mid, idx * 2);
        }
        if qr > mid {
            self._update(ql, qr, tag, mid + 1, r, idx * 2 + 1);
        }
        self.push_up(idx);
    }

    fn find_last_zero(&mut self, start: usize) -> i32 {
        if start > self.n {
            return -1;
        }
        self._find(start, self.n, 0, 1, self.n, 1)
    }

    fn _find(&mut self, ql: usize, qr: usize, val: i32, l: usize, r: usize, idx: usize) -> i32 {
        if l > qr || r < ql || self.tree[idx].min_val > val || self.tree[idx].max_val < val {
            return -1;
        }

        if l == r {
            return if self.tree[idx].min_val == val {
                l as i32
            } else {
                -1
            };
        }

        self.push_down(idx);
        let mid = (l + r) / 2;

        let right_res = self._find(ql, qr, val, mid + 1, r, idx * 2 + 1);
        if right_res != -1 {
            return right_res;
        }

        self._find(ql, qr, val, l, mid, idx * 2)
    }
}

struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let sign = |x: i32| if x % 2 == 0 { 1 } else { -1 };

        let mut prefix_sum = vec![0; n];
        let mut pos_map: HashMap<i32, VecDeque<usize>> = HashMap::new();

        for i in 0..n {
            if i > 0 {
                prefix_sum[i] = prefix_sum[i - 1];
            }
            let positions = pos_map.entry(nums[i]).or_insert_with(VecDeque::new);

            if positions.is_empty() {
                prefix_sum[i] += sign(nums[i]);
            }
            positions.push_back(i + 1);
        }

        let mut seg_tree = SegmentTree::new(&prefix_sum);
        let mut max_len = 0;

        for i in 0..n {
            let start_search = i + 1 + max_len as usize;

            if start_search <= n {
                let last_pos = seg_tree.find_last_zero(start_search);
                if last_pos != -1 {
                    max_len = max(max_len, last_pos - i as i32);
                }
            }

            let num = nums[i];

            let positions = pos_map.get_mut(&num).unwrap();
            positions.pop_front();
            let next_pos = positions.front().copied().unwrap_or(n + 1);

            let delta = -sign(num);
            if i + 2 <= next_pos {
                seg_tree.range_add(i + 2, next_pos - 1, delta);
            }
        }

        max_len
    }
}
