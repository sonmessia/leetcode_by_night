struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = [0; 51];
        let k = k as usize;
        let n = nums.len();

        if n == k {
            return *nums.iter().max().unwrap_or(&-1);
        }

        for i in 0..n {
            freq[nums[i] as usize] += 1;
        }

        if k == 1 {
            return freq
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, &c)| if c == 1 { Some(i as i32) } else { None })
                .unwrap_or(-1);
        }

        let mut ans = -1;

        if freq[nums[0] as usize] == 1 {
            ans = ans.max(nums[0]);
        }

        if freq[nums[n - 1] as usize] == 1 {
            ans = ans.max(nums[n - 1]);
        }

        ans
    }
}
