struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let (mut close, mut insert_count) = (0, 0);

        for c in s.chars() {
            if c == '(' {
                close += 2;
                if close % 2 == 1 {
                    insert_count += 1;
                    close -= 1;
                }
            } else {
                close -= 1;
                if close < 0 {
                    insert_count += 1;
                    close = 1;
                }
            }
        }

        close + insert_count
    }
}
