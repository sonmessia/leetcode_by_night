struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort();
        let mut ans: i64 = -1;
        let mut curr_sum: i64 = 0;

        for num in nums {
            if curr_sum > num as i64 {
                ans = (ans.max(curr_sum + num as i64));
            }
            curr_sum = curr_sum + num as i64;
        }
        ans
    }
}
