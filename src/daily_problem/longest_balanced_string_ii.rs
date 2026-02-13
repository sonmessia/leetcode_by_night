use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_bytes();

        max(
            Self::trio(s),
            max(
                Self::mono(s),
                max(
                    Self::duo(s, b'a', b'b'),
                    max(Self::duo(s, b'a', b'c'), Self::duo(s, b'b', b'c')),
                ),
            ),
        )
    }

    fn mono(s: &[u8]) -> i32 {
        let mut ans = 1;
        let mut cnt = 1;
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }
            ans = max(ans, cnt);
        }
        ans as i32
    }

    fn duo(s: &[u8], c1: u8, c2: u8) -> i32 {
        let mut pos = HashMap::new();
        pos.insert(0, -1);

        let mut ans = 0;
        let mut delta = 0;

        for (i, &ch) in s.iter().enumerate() {
            if ch != c1 && ch != c2 {
                pos = HashMap::new();
                pos.insert(0, i as i32);
                delta = 0;
                continue;
            }

            if ch == c1 {
                delta += 1;
            } else {
                delta -= 1;
            }

            pos.entry(delta)
                .and_modify(|&mut old_idx| ans = max(ans, i as i32 - old_idx))
                .or_insert(i as i32);
        }
        ans
    }

    fn trio(s: &[u8]) -> i32 {
        let mut cnt = [0; 3];
        let mut pos = HashMap::new();

        pos.insert((0, 0), -1);

        let mut ans = 0;

        for (i, &ch) in s.iter().enumerate() {
            match ch {
                b'a' => cnt[0] += 1,
                b'b' => cnt[1] += 1,
                b'c' => cnt[2] += 1,
                _ => {}
            }

            let key = (cnt[1] - cnt[0], cnt[2] - cnt[0]);

            pos.entry(key)
                .and_modify(|&mut old_idx| ans = max(ans, i as i32 - old_idx))
                .or_insert(i as i32);
        }
        ans
    }
}
