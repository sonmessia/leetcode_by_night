struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd_count = 0;
        let mut even_count = 1;
        let mut prefix_sum = 0;
        let mut result = 0i64;

        for num in arr {
            prefix_sum += num;
            if prefix_sum % 2 == 0 {
                result += odd_count;
                even_count += 1;
            } else {
                result += even_count;
                odd_count += 1;
            }
        }

        (result % 1_000_000_007 as i64) as i32
    }
}
