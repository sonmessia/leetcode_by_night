struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        fn dfs(nums: &Vec<i32>, left: usize, right: usize) -> i32 {
            if left == right {
                return nums[left];
            }
            let pick_left = nums[left] - dfs(nums, left + 1, right);
            let pick_right = nums[right] - dfs(nums, left, right - 1);
            pick_left.max(pick_right)
        }
        let n = nums.len();
        dfs(&nums, 0, n - 1) >= 0
    }
}

fn main() {
    let nums = vec![1, 5, 2];
    println!("{}", Solution::predict_the_winner(nums));
    let nums = vec![1, 5, 233, 7];
    println!("{}", Solution::predict_the_winner(nums));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_predict_the_winner() {
        let nums = vec![1, 5, 2];
        assert_eq!(Solution::predict_the_winner(nums), false);
        let nums = vec![1, 5, 233, 7];
        assert_eq!(Solution::predict_the_winner(nums), true);
    }
}
