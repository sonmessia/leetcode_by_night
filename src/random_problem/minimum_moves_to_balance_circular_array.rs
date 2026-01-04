struct Solution;

impl Solution {
    pub fn min_moves(balance: Vec<i32>) -> i32 {
        let n = balance.len();
        let mut j = -1;

        for i in 0..n {
            if balance[i] < 0 {
                j = i as i32;
            }
        }

        if j < 0 {
            return 0;
        }

        if balance.iter().map(|&x| x as i64).sum::<i64>() < 0 {
            return -1;
        }

        let mut ans: i64 = 0;
        let mut d = 0;

        let mut val = balance[j as usize];

        while val < 0 {
            d += 1;
            let left = (j as usize + d) % n;
            let right = (j as usize - d + n) % n;
            let mut s = 0;
            if left == right {
                s = balance[left];
            } else {
                s = balance[left] + balance[right];
            }
            ans += s.min(-val) as i64 * d as i64;
            val += s;
        }

        ans as i32
    }
}
