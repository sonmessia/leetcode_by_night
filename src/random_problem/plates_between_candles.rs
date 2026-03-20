struct Solution;

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; s.len() + 1];
        let mut left_candle = vec![-1; s.len()];
        let mut right_candle = vec![-1; s.len()];

        for (i, c) in s.chars().enumerate() {
            prefix_sum[i + 1] = prefix_sum[i] + if c == '*' { 1 } else { 0 };
            left_candle[i] = if c == '|' {
                i as i32
            } else {
                if i > 0 { left_candle[i - 1] } else { -1 }
            };
        }

        for (i, c) in s.chars().rev().enumerate() {
            let idx = s.len() - 1 - i;
            right_candle[idx] = if c == '|' {
                idx as i32
            } else {
                if idx < s.len() - 1 {
                    right_candle[idx + 1]
                } else {
                    -1
                }
            };
        }

        // for i in (0..s.len()).rev() {
        //     right_candle[i] = if s.chars().nth(i).unwrap() == '|' {
        //         i as i32
        //     } else {
        //         if i < s.len() - 1 {
        //             right_candle[i + 1]
        //         } else {
        //             -1
        //         }
        //     };
        // }

        queries
            .into_iter()
            .map(|q| {
                let left = right_candle[q[0] as usize];
                let right = left_candle[q[1] as usize];
                if left != -1 && right != -1 && left < right {
                    prefix_sum[right as usize + 1] - prefix_sum[left as usize + 1]
                } else {
                    0
                }
            })
            .collect()
    }
}
