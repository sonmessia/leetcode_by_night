struct Solution;

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        let mut factory = factory;

        robot.sort_unstable();
        factory.sort_unstable_by_key(|f| f[0]);

        let n = robot.len();
        let m = factory.len();

        let inf: i64 = 1_000_000_000_000_i64;
        let mut dp = vec![vec![inf; m + 1]; n + 1];

        for j in 0..=m {
            dp[0][j] = 0;
        }

        for j in 1..=m {
            let factory_pos = factory[j - 1][0] as i64;
            let factory_cap = factory[j - 1][1] as usize;

            for i in 0..=n {
                dp[i][j] = dp[i][j - 1];

                let mut total_dist = 0i64;
                for k in 1..=factory_cap {
                    if i < k {
                        break;
                    }

                    let robot_pos = robot[i - k] as i64;
                    total_dist += (robot_pos - factory_pos).abs();

                    if dp[i - k][j - 1] != inf {
                        dp[i][j] = dp[i][j].min(dp[i - k][j - 1] + total_dist);
                    }
                }
            }
        }

        dp[n][m]
    }
}
