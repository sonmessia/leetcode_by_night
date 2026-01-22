struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut decrease = true;

        while decrease && nums.len() > 1 {
            decrease = false;
            let mut min_sum = nums[0] + nums[1];
            let mut j = 0;

            for i in 1..nums.len() {
                if nums[i - 1] > nums[i] {
                    decrease = true;
                }

                let sum = nums[i - 1] + nums[i];
                if min_sum > sum {
                    min_sum = sum;
                    j = i - 1;
                }
            }
            if decrease {
                cnt += 1;
                nums.splice(j..=j + 1, std::iter::once(min_sum));
            }
        }

        cnt
    }
}
