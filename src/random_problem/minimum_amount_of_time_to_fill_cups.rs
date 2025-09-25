struct Solution;

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_unstable();
        let sum: i32 = amount.iter().sum();
        std::cmp::max(amount[2], (sum + 1) / 2)
    }
}
