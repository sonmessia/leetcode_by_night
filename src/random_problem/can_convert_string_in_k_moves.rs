struct Solution;

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut move_counts = vec![0; 26];
        for (sc, tc) in s.bytes().zip(t.bytes()) {
            let diff = if tc >= sc { tc - sc } else { tc + 26 - sc };
            if diff > 0 {
                move_counts[(diff - 1) as usize] += 1;
            }
        }
        for (i, &count) in move_counts.iter().enumerate() {
            let required_moves = (i as i32 + 1) + (count as i32 - 1) * 26;
            if required_moves > k {
                return false;
            }
        }
        true
    }
}
