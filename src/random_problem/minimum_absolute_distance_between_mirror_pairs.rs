struct Solution;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        fn reverse(mut num: i32) -> i32 {
            let mut rev = 0;
            while num > 0 {
                rev = rev * 10 + num % 10;
                num /= 10;
            }
            rev
        }

        use std::collections::HashMap;

        let mut mp = HashMap::new();
        let n = nums.len();

        let mut min_distance = i32::MAX;

        for j in 0..n {
            match mp.get(&nums[j]) {
                Some(i) => {
                    min_distance = min_distance.min((j as i32 - *i as i32).abs());
                }
                _ => (),
            }
            mp.insert(reverse(nums[j]), j);
        }

        if min_distance == i32::MAX {
            -1
        } else {
            min_distance
        }
    }
}
