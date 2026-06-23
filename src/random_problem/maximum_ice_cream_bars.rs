struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();

        for (idx, &cost) in costs.iter().enumerate() {
            if cost > coins {
                return idx as i32;
            }
            coins -= cost;
        }

        costs.len() as i32
    }
}

