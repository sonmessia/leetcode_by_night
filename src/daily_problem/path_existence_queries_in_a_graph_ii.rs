struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut arr: Vec<(i32, usize)> = Vec::new();
        let mut up = vec![0; n];

        for i in 0..n {
            arr.push((nums[i as usize], i));
        }

        arr.sort_by(|a, b| a.0.cmp(&b.0));

        let mut points = vec![0; n];

        for i in 0..n {
            points[arr[i].1] = i;
        }

        let mut right = 0usize;
        for left in 0..n as usize {
            if right < left {
                right = left;
            }

            while right + 1 < n && arr[right + 1].0 - arr[left].0 <= max_diff {
                right += 1;
            }

            up[left] = right;
        }

        let log = ((n as f64).log2().ceil()) as usize + 1;
        let mut dp = vec![vec![0; n + 1]; log];

        for i in 0..n {
            dp[0][i] = up[i];
        }

        for i in 1..log {
            for j in 0..n {
                dp[i][j] = dp[i - 1][dp[i - 1][j]];
            }
        }

        let mut ans = vec![];
        for query in queries {
            let (u, v) = (query[0] as usize, query[1] as usize);
            if u == v {
                ans.push(0);
                continue;
            }

            let (start, end) = (points[u].min(points[v]), points[u].max(points[v]));

            let mut node = start;
            let mut steps = 0;
            for i in (0..log).rev() {
                if dp[i][node] < end {
                    node = dp[i][node];
                    steps += 1 << i;
                }
            }

            if up[node] < end {
                ans.push(-1);
            } else {
                ans.push(steps + 1);
            }
        }

        ans
    }
}
