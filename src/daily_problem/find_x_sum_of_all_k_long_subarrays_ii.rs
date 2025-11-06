use std::collections::{BTreeSet, HashMap};
struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let n = nums.len();
        let k = k as usize;
        let x = x as usize;

        if n < k {
            return vec![];
        }
        let mut ans = vec![0i64; n - k + 1];

        let mut freq: HashMap<i64, i64> = HashMap::new();

        let mut scores: BTreeSet<(i64, i64)> = BTreeSet::new();

        let calculate_score = |scores: &BTreeSet<(i64, i64)>, x: usize| -> i64 {
            let mut sum = 0;
            for (count, value) in scores.iter().take(x) {
                sum += count * value;
            }
            sum
        };

        let update_windows = |val: i64,
                              freq: &mut HashMap<i64, i64>,
                              scores: &mut BTreeSet<(i64, i64)>,
                              is_adding: bool| {
            let old_cnt = *freq.get(&val).unwrap_or(&0);

            if old_cnt > 0 {
                scores.remove(&(-old_cnt, -val));
            }

            let new_cnt = if is_adding { old_cnt + 1 } else { old_cnt - 1 };

            if new_cnt > 0 {
                scores.insert((-new_cnt, -val));
                freq.insert(val, new_cnt);
            } else {
                freq.remove(&val);
            }
        };

        for i in 0..k {
            *freq.entry(nums[i] as i64).or_insert(0) += 1;
        }

        for (&val, &count) in freq.iter() {
            scores.insert((-count, -val));
        }

        ans[0] = calculate_score(&scores, x);

        for i in 1..ans.len() {
            let val_to_remove = nums[i - 1] as i64;
            let val_to_add = nums[i + k - 1] as i64;

            update_windows(val_to_remove, &mut freq, &mut scores, false);
            update_windows(val_to_add, &mut freq, &mut scores, true);
            ans[i] = calculate_score(&scores, x);
        }

        ans
    }
}
