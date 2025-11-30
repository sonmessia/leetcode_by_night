struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        use std::collections::HashMap;

        let total_mod = nums.iter().fold(0, |acc, &x| (acc + x) % p);

        if total_mod == 0 {
            return 0;
        }

        let mut prefix_mod = 0;
        let mut mod_index_map = HashMap::new();
        mod_index_map.insert(0, -1);
        let mut min_length = nums.len() as i32;

        for (i, &num) in nums.iter().enumerate() {
            prefix_mod = (prefix_mod + num) % p;
            let target_mod = (prefix_mod - total_mod + p) % p;

            if let Some(&prev_index) = mod_index_map.get(&target_mod) {
                min_length = min_length.min(i as i32 - prev_index);
            }

            mod_index_map.insert(prefix_mod, i as i32);
        }

        if min_length as usize == nums.len() {
            -1
        } else {
            min_length
        }
    }
}
