use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr: Vec<i32> = arr.iter().cloned().collect();
        sorted_arr.sort();
        let mut rank_map = HashMap::new();
        let mut rank = 1;

        for v in sorted_arr {
            if !rank_map.contains_key(&v) {
                rank_map.insert(v, rank);
                rank += 1;
            }
        }

        for i in 0..arr.len() {
            match rank_map.get(&arr[i]) {
                Some(&val) => arr[i] = val,
                None => break,
            }
        }
        arr
    }
}
