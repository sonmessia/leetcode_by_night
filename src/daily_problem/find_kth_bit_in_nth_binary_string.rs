struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut s = "0".to_string();
        for _ in 1..n {
            let mut t = s.clone();
            t.push('1');
            t.push_str(
                &s.chars()
                    .rev()
                    .map(|c| if c == '0' { '1' } else { '0' })
                    .collect::<String>(),
            );
            s = t;
        }
        s.chars().nth((k - 1) as usize).unwrap_or('0')
    }
}
