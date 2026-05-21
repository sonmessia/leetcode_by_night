use std::collections::HashSet;

struct Solution;

impl Solution {
    fn precompute_prefix(mut num: i32, set: &mut HashSet<i32>) {
        while num > 0 {
            set.insert(num);
            num /= 10;
        }
    }

    fn check_exist_prefix(num: i32, set: &HashSet<i32>) -> i32 {
        let mut num = num;

        let mut cnt = 0;
        while num > 0 {
            if set.contains(&num) {
                cnt += 1;
            }

            num /= 10;
        }

        cnt
    }
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        let mut ans = 0;

        for num in arr1.into_iter() {
            Self::precompute_prefix(num, &mut set);
        }

        for num in arr2.into_iter() {
            ans = ans.max(Self::check_exist_prefix(num, &set));
        }
        ans
    }
}
