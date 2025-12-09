struct Solution;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        const MOD: i64 = 1_000_000_007;

        let mut ans: i64 = 0;

        let (mut count_left, mut count_right) =
            (HashMap::<i32, i32>::new(), HashMap::<i32, i32>::new());

        for &num in &nums {
            *count_right.entry(num).or_insert(0) += 1;
        }

        for &num in &nums {
            if let Some(v) = count_right.get_mut(&num) {
                *v -= 1;
            }

            let left = *count_left.get(&(num * 2)).unwrap_or(&0) as i64;
            let right = *count_right.get(&(num * 2)).unwrap_or(&0) as i64;

            ans = (ans + (left * right) % MOD) % MOD;

            *count_left.entry(num).or_insert(0) += 1;
        }

        ans as i32
    }
}
