struct Solution;

impl Solution {
    pub fn max_consecutive_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 1;
        let (mut prev, mut len1, mut len2) = (-2, 0, 0);

        for num in nums {
            (len1, len2) = match num - prev {
                0 => (len1, len2.max(len1 + 1)),
                1 => (len2.max(len1 + 1), len2 + 1),
                2 => (len2 + 1, 1),
                _ => (1, 1),
            };
            prev = num;
            ans = ans.max(len1.max(len2));
        }

        ans
    }
}
