struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();
        let k = index_diff as usize;
        let t = value_diff as i64;

        for (i, &num) in nums.iter().enumerate() {
            let num = num as i64;

            if let Some(&x) = set.range(num - t..).next() {
                if x <= num + t {
                    return true;
                }
            }

            set.insert(num);

            if i >= k {
                set.remove(&(nums[i - k] as i64));
            }
        }

        false
    }
}
