struct DSU {
    size: Vec<usize>,
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            size: vec![1; n],
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

struct Solution;

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n_usize = n as usize;
        let (mut musts, mut not_musts) = (Vec::new(), Vec::new());

        for edge in edges {
            if edge[3] == 1 {
                musts.push(edge);
            } else {
                not_musts.push(edge);
            }
        }

        let mut dsu = DSU::new(n_usize);
        let mut mins = i32::MAX;

        for edge in &musts {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];

            if dsu.is_same(u, v) {
                return -1;
            }

            mins = mins.min(w);
            dsu.union(u, v);
        }

        let mut new_not_musts = Vec::new();
        for edge in not_musts {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if dsu.is_same(u, v) {
                continue;
            }
            new_not_musts.push(edge);
        }

        new_not_musts.sort_by(|a, b| b[2].cmp(&a[2]));

        let mut allowed_in = Vec::new();

        for edge in new_not_musts {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let weight = edge[2];

            if dsu.is_same(u, v) {
                continue;
            }
            dsu.union(u, v);
            allowed_in.push(weight);
        }

        let parent_0 = dsu.find(0);
        for x in 1..n_usize {
            if dsu.find(x) != parent_0 {
                return -1;
            }
        }

        allowed_in.sort_unstable();

        let limit = (k as usize).min(allowed_in.len());
        for i in 0..limit {
            allowed_in[i] *= 2;
        }

        if !allowed_in.is_empty() {
            let min_allowed = *allowed_in.iter().min().unwrap();
            mins = mins.min(min_allowed);
        }

        mins
    }
}

