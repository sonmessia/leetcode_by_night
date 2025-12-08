struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for a in 1..=n {
            for b in a..=n {
                let c_squared = a * a + b * b;
                let c = (c_squared as f64).sqrt() as i32;
                if c * c == c_squared && c <= n {
                    if a == b {
                        count += 1;
                    } else {
                        count += 2;
                    }
                }
            }
        }
        count
    }
}
