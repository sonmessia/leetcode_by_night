use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let m = classroom.len();
        let n = classroom[0].len();

        let mut id = vec![vec![0usize; n]; m];

        let (mut sx, mut sy, mut cnt) = (0usize, 0usize, 0);

        for i in 0..m {
            for j in 0..n {
                let c = classroom[i].as_bytes()[j] as char;

                if c == 'S' {
                    sx = i;
                    sy = j;
                } else if c == 'L' {
                    id[i][j] = 1 << cnt;
                    cnt += 1;
                }
            }
        }

        if cnt == 0 {
            return 0;
        }

        let target_mask = (1 << cnt) - 1;

        let mut dp = vec![vec![vec![-1; target_mask + 1]; n]; m];

        dp[sx][sy][0] = energy;

        #[derive(Clone)]
        struct State {
            x: usize,
            y: usize,
            mask: usize,
            e: i32,
            steps: i32,
        }

        let mut q = VecDeque::new();

        q.push_back(State {
            x: sx,
            y: sy,
            mask: 0,
            e: energy,
            steps: 0,
        });

        while let Some(State {
            x,
            y,
            mask,
            e,
            steps,
        }) = q.pop_front()
        {
            if mask == target_mask {
                return steps;
            }

            if e == 0 {
                continue;
            }

            for &(dx, dy) in &DIRECTIONS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                let c = classroom[nx].as_bytes()[ny] as char;

                if c == 'X' {
                    continue;
                }

                let nenergy = if c == 'R' { energy } else { e - 1 };

                let nmask = mask | id[nx][ny];

                if nenergy > dp[nx][ny][nmask] {
                    dp[nx][ny][nmask] = nenergy;

                    q.push_back(State {
                        x: nx,
                        y: ny,
                        mask: nmask,
                        e: nenergy,
                        steps: steps + 1,
                    });
                }
            }
        }

        -1
    }
}
