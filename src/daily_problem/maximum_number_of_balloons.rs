struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut freq = [0; 5];
        for c in text.chars() {
            match c {
                'b' => freq[0] += 1,
                'a' => freq[1] += 1,
                'l' => freq[2] += 1,
                'o' => freq[3] += 1,
                'n' => freq[4] += 1,
                _ => (),
            }
        }

        freq[2] /= 2;
        freq[3] /= 2;
        *freq.iter().min().unwrap() as i32
    }
}
