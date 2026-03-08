struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut result = String::new();
        for i in 0..n {
            let c = if nums[i].chars().nth(i).unwrap() == '0' {
                '1'
            } else {
                '0'
            };
            result.push(c);
        }
        result
    }
}
