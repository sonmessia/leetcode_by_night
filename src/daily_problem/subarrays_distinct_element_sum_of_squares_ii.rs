const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut tree = SegmentTree::new(n);
        let mut last = vec![-1i32; 100_001];
        let mut ans = 0i64;

        for r in 0..n {
            let x = nums[r] as usize;
            let prev = last[x];

            tree.add(prev + 1, r as i32, 1);
            ans = (ans + tree.sum_of_squares()) % MOD;

            last[x] = r as i32;
        }

        ans as i32
    }
}

struct SegmentTree {
    sum: Vec<i64>,
    sq: Vec<i64>,
    lazy: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            sum: vec![0; 4 * n],
            sq: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
            n,
        }
    }

    fn add(&mut self, ql: i32, qr: i32, delta: i64) {
        self.add_range(1, 0, self.n as i32 - 1, ql, qr, delta);
    }

    fn sum_of_squares(&self) -> i64 {
        self.sq[1]
    }

    fn add_range(&mut self, node: usize, l: i32, r: i32, ql: i32, qr: i32, delta: i64) {
        if ql <= l && r <= qr {
            self.apply(node, l, r, delta);
            return;
        }

        self.push(node, l, r);

        let mid = (l + r) / 2;
        if ql <= mid {
            self.add_range(node * 2, l, mid, ql, qr, delta);
        }
        if qr > mid {
            self.add_range(node * 2 + 1, mid + 1, r, ql, qr, delta);
        }

        self.pull(node);
    }

    fn apply(&mut self, node: usize, l: i32, r: i32, delta: i64) {
        let len = (r - l + 1) as i64;

        self.sq[node] =
            (self.sq[node] + 2 * delta % MOD * self.sum[node] + delta * delta % MOD * len) % MOD;
        self.sum[node] = (self.sum[node] + delta * len) % MOD;
        self.lazy[node] = (self.lazy[node] + delta) % MOD;
    }

    fn push(&mut self, node: usize, l: i32, r: i32) {
        if self.lazy[node] == 0 {
            return;
        }

        let mid = (l + r) / 2;
        let delta = self.lazy[node];
        self.apply(node * 2, l, mid, delta);
        self.apply(node * 2 + 1, mid + 1, r, delta);
        self.lazy[node] = 0;
    }

    fn pull(&mut self, node: usize) {
        self.sum[node] = (self.sum[node * 2] + self.sum[node * 2 + 1]) % MOD;
        self.sq[node] = (self.sq[node * 2] + self.sq[node * 2 + 1]) % MOD;
    }
}
