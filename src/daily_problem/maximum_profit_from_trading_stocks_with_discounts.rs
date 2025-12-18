struct Solution;

impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let n = n as usize;
        let budget = budget as usize;
        let mut g = vec![vec![]; n];
        for edge in hierarchy {
            g[edge[0] as usize - 1].push(edge[1] as usize - 1);
        }

        fn dfs(
            u: usize,
            present: &[i32],
            future: &[i32],
            g: &[Vec<usize>],
            budget: usize,
        ) -> (Vec<i32>, Vec<i32>, usize) {
            let cost = present[u] as usize;
            let d_cost = present[u] as usize / 2;

            let mut dp0 = vec![0; budget + 1];
            let mut dp1 = vec![0; budget + 1];

            let mut sub_profit0 = vec![0; budget + 1];
            let mut sub_profit1 = vec![0; budget + 1];

            let mut u_size = cost;

            for &v in &g[u] {
                let (child_dp0, child_dp1, child_size) = dfs(v, present, future, g, budget);

                u_size += child_size;

                for i in (0..=budget).rev() {
                    for sub in 0..=child_size.min(i) {
                        if i >= sub {
                            sub_profit0[i] =
                                sub_profit0[i].max(sub_profit0[i - sub] + child_dp0[sub]);
                            sub_profit1[i] =
                                sub_profit1[i].max(sub_profit1[i - sub] + child_dp1[sub]);
                        }
                    }
                }
            }

            for i in 0..=budget {
                dp0[i] = sub_profit0[i];
                dp1[i] = sub_profit1[i];

                if i >= d_cost {
                    dp1[i] = dp1[i].max(sub_profit1[i - d_cost] + future[u] - d_cost as i32);
                }

                if i >= cost {
                    dp0[i] = dp0[i].max(sub_profit0[i - cost] + future[u] - cost as i32);
                }
            }

            (dp0, dp1, u_size)
        }

        let (dp0, _, _) = dfs(0, &present, &future, &g, budget);
        dp0[budget]
    }
}
