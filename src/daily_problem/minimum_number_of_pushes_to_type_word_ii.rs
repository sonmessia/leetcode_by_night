struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freq = vec![0; 26];

        for c in word.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }

        freq.sort_unstable_by(|a, b| b.cmp(a));

        // println!("{:?}", freq);

        let mut ans = 0;

        for i in 0..26 {
            ans += freq[i] * (i as i32 / 8 + 1);
        }

        ans
    }
}
