struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        fn helper(n: i32) -> i32 {
            let mut ans = n;
            for i in 0..=32 {
                let candidate = (1 << i) ^ n;
                // println!("candidate: {}", candidate);
                if candidate | (candidate + 1) == n {
                    ans = ans.min(candidate);
                }
            }
            return ans;
        }
        let mut ans = vec![];

        for num in nums {
            if num == 2 {
                ans.push(-1);
            } else {
                ans.push(helper(num));
                // println!();
            }
        }

        ans
    }
}
