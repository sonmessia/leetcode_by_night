struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let cost1 = cost1 as i64;
        let cost2 = cost2 as i64;
        let cost_both = cost_both as i64;
        let need1 = need1 as i64;
        let need2 = need2 as i64;

        (need1 * cost1 + need2 * cost2)
            .min((need1.max(need2)) * cost_both)
            .min(cost_both * need1 + (need2 - need1).abs() * cost2)
            .min(cost_both * need2 + (need1 - need2).abs() * cost1)
    }
}
