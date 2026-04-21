use std::collections::HashMap;
struct Solution;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let parent = (0..size).collect();
        let rank = vec![1; size];
        Self { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let mut union_find = UnionFind::new(n);
        for swap in allowed_swaps {
            union_find.union(swap[0] as usize, swap[1] as usize);
        }

        let mut components: HashMap<usize, HashMap<i32, i32>> = HashMap::new();
        let mut ans = 0;

        for (i, &num) in source.iter().enumerate() {
            let root = union_find.find(i);
            *components
                .entry(root)
                .or_insert_with(HashMap::new)
                .entry(num)
                .or_insert(0) += 1;
        }

        for (i, &num) in target.iter().enumerate() {
            let root = union_find.find(i);
            if let Some(counts) = components.get_mut(&root) {
                if let Some(count) = counts.get_mut(&num) {
                    if *count > 0 {
                        *count -= 1;
                        continue;
                    }
                }
            }
            ans += 1;
        }
        ans
    }
}
