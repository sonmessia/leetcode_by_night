struct Solution;

impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        queries.iter().for_each(|q| {
            (q[0] as usize..=q[1] as usize)
                .step_by(q[2] as usize)
                .for_each(|i| nums[i] = (nums[i] as i64 * q[3] as i64 % 1000000007) as i32);
        });
        nums.iter().fold(0, |acc, &x| acc ^ x)
    }
}
