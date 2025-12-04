struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let (mut ans, mut cnt, n) = (0, 1, directions.len());

        let directions = directions.as_bytes();
        let mut last = directions[0];

        for i in 1..n {
            let curr = directions[i];

            if last == curr {
                if last != b'S' {
                    cnt += 1;
                }
                continue;
            }

            if last == b'S' && curr == b'L' || last == b'R' && curr == b'S' {
                ans += cnt;
                last = b'S';
                cnt = 1;
                continue;
            }

            if last == b'R' && curr == b'L' {
                ans += cnt + 1;
                last = b'S';
                cnt = 1;
                continue;
            }

            last = curr;
            cnt = 1;
        }
        ans
    }
}
