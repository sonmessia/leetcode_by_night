struct Solution;

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut sorted_cost = cost;
        sorted_cost.sort_unstable_by(|a, b| b.cmp(a));

        // let mut total_cost = 0;
        // for (i, &c) in sorted_cost.iter().enumerate() {
        //     if (i + 1) % 3 != 0 {
        //         total_cost += c;
        //     }
        // }
        //
        // total_cost

        sorted_cost
            .chunks(3)
            .map(|chunk| chunk.iter().take(2).sum::<i32>())
            .sum()
    }
}
