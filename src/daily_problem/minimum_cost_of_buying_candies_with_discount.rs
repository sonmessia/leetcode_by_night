struct Solution;

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut sorted_cost = cost;
        sorted_cost.sort_unstable_by(|a, b| b.cmp(a));

        sorted_cost
            .chunks(3)
            .map(|chunk| chunk.iter().take(2).sum::<i32>())
            .sum()
    }
}
