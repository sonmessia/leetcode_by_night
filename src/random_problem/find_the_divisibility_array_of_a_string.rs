struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let n = word.len();
        let mut result = vec![0; n];
        let mut current_mod = 0;
        let base = 10;
        let m = m as i64;

        for (i, ch) in word.chars().enumerate() {
            let char_digit = ch.to_digit(10).unwrap() as i64;
            current_mod = (current_mod * base + char_digit) % m;

            if current_mod == 0 {
                result[i] = 1;
            }
        }

        result
    }
}
