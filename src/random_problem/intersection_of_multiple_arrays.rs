struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count_map = vec![0; 1005];
        let n = nums.len() as i32;

        for arr in nums {
            for &num in &arr {
                count_map[num as usize] += 1;
            }
        }

        let mut result = Vec::new();
        for i in 1..=1000 {
            if count_map[i] == n {
                result.push(i as i32);
            }
        }
        result.sort();
        result
    }
}
