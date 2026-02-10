struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let n = nums.len();

        for i in 0..n {
            if n - i <= ans {
                break;
            }

            let (mut even, mut odd) = (0, 0);
            let mut seen = [false; 100_005];

            for j in i..n {
                let curr_sum = nums[j] as usize;

                if !seen[curr_sum] {
                    seen[curr_sum] = true;

                    if curr_sum % 2 == 0 {
                        even += 1;
                    } else {
                        odd += 1;
                    }
                }

                if even == odd {
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as i32
    }
}
