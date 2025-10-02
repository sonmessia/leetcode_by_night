use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if max_choosable_integer >= desired_total || desired_total <= 0 {
            return true;
        }

        let sum: i64 = (1 + max_choosable_integer) as i64 * max_choosable_integer as i64 / 2;
        if sum < desired_total as i64 {
            return false;
        }

        let mut memo = HashMap::new();
        fn dfs(
            used_mask: i32,
            target: i32,
            memo: &mut HashMap<i32, bool>,
            max_choosable_integer: i32,
        ) -> bool {
            if target <= 0 {
                return false;
            }

            if let Some(&result) = memo.get(&used_mask) {
                return result;
            }

            for i in 1..=max_choosable_integer {
                let current_bit = 1 << (i - 1);
                if used_mask & current_bit == 0
                    && !dfs(
                        used_mask | current_bit,
                        target - i,
                        memo,
                        max_choosable_integer,
                    )
                {
                    memo.insert(used_mask, true);
                    return true;
                }
            }

            memo.insert(used_mask, false);
            false
        }

        dfs(0, desired_total, &mut memo, max_choosable_integer)
    }
}

fn main() {
    let max_choosable_integer = 5;
    let desired_total = 6;
    let result = Solution::can_i_win(max_choosable_integer, desired_total);
    println!("Can I win? {}", result); // Output: Can I win? false
}
