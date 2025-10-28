struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let (mut prefix_sum, mut suffix_sum) = (vec![0; nums.len() + 1], vec![0; nums.len() + 1]);

        for i in 1..=nums.len() {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }

        for i in (0..nums.len()).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + nums[i];
        }

        let mut ans = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                let left_sum = prefix_sum[i];
                let right_sum = suffix_sum[i + 1];
                if left_sum == right_sum {
                    ans += 2;
                }
                if (left_sum - right_sum).abs() == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let nums = vec![1, 0, 2, 0, 3];
    let result = Solution::count_valid_selections(nums);
    println!("Result: {}", result);
}
