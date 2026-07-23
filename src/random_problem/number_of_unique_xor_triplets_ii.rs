use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let mut ans = HashSet::new();

        let mut set = HashSet::new();

        let n = nums.len();

        for i in 0..n {
            for j in i..n {
                set.insert(nums[i] ^ nums[j]);
            }
        }

        // println!("{:?}", ans);

        for unique in set.iter() {
            for i in 0..n {
                ans.insert(unique ^ nums[i]);
            }
        }

        ans.len() as i32
    }
}
