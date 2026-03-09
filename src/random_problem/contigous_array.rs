struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        let mut count = 0;
        let mut max_length = 0;
        for (i, num) in nums.iter().enumerate() {
            if *num == 0 {
                count -= 1;
            } else {
                count += 1;
            }
            if let Some(&prev_index) = map.get(&count) {
                max_length = max_length.max(i as i32 - prev_index);
            } else {
                map.insert(count, i as i32);
            }
        }
        max_length
    }
}
