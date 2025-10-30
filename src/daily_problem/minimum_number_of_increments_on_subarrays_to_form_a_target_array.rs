struct Solution;

impl Solution {
    pub fn min_number_of_increments(target: Vec<i32>) -> i32 {
        let mut ans = target[0];
        for i in 1..target.len() {
            if target[i] > target[i - 1] {
                ans += target[i] - target[i - 1];
            }
        }
        ans
    }
}
