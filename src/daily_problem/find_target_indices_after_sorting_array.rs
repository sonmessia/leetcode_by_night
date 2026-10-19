struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut frequency = vec![0; 101];

        for &num in &nums {
            frequency[num as usize] += 1;
        }

        let mut index = 0;

        for num in 0..=100 {
            if num as i32 != target {
                index += frequency[num];
            } else {
                for _ in 0..frequency[num] {
                    result.push(index);
                    index += 1;
                }
            }
        }
        result
    }
}
