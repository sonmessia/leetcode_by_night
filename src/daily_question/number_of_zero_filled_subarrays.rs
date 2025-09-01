struct Solution {}

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        nums.iter()
            .fold((0i64, 0i64), |(ans, sub), &num| {
                let sub = if num == 0 { sub + 1 } else { 0 };
                (ans + sub, sub)
            })
            .0
    }
}

fn main() {
    let nums = vec![1, 0, 0, 2, 0, 0, 0, 3];
    let result = Solution::zero_filled_subarray(nums);
    println!("Number of zero-filled subarrays: {}", result);
}
