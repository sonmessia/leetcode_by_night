struct Solution;

impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let total_ones = nums.iter().filter(|&&x| x == 1).count() as i32;

        let mut ans = Vec::new();
        let mut left_zeros = 0;
        let mut left_ones = 0;

        ans.push(total_ones);

        for i in 1..=n {
            if nums[i - 1] == 0 {
                left_zeros += 1;
            } else {
                left_ones += 1;
            }

            let right_ones = total_ones - left_ones;
            let score = left_zeros + right_ones;

            ans.push(score);
        }

        let max_score = *ans.iter().max().unwrap_or(&0);

        ans.iter()
            .enumerate()
            .filter_map(|(i, &score)| {
                if score == max_score {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    let nums = vec![1, 0, 0, 0];
    let result = Solution::max_score_indices(nums);
    println!("Result: {:?}", result); // Expected output: [1, 2, 3, 2, 3]
}
