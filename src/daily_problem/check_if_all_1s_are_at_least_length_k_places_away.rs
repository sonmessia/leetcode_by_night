struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last_one_index: Option<usize> = None;

        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                if let Some(last_index) = last_one_index {
                    if i - last_index - 1 < k as usize {
                        return false;
                    }
                }
                last_one_index = Some(i);
            }
        }

        true
    }
}
