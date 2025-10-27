struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let (mut prev, mut ans) = (0, 0);

        for b in bank.iter() {
            let count = b.chars().filter(|&c| c == '1').count() as i32;
            if count > 0 {
                ans += prev * count;
                prev = count;
            }
        }

        ans
    }
}
