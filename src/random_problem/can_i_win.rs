struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        fn backtrack(
            choice_array: &Vec<i32>,
            curr_sum: &mut i32,
            turn: usize,
            desired_total: &mut i32,
            used: &mut Vec<bool>,
        ) -> bool {
            if curr_sum >= desired_total {
                return turn % 2 == 0;
            }

            let mut force_win = true;
            for i in 0..choice_array.len() {
                if used[choice_array[i] as usize] {
                    continue;
                }
                used[choice_array[i] as usize] = true;
                *curr_sum += choice_array[i];
                let res = backtrack(choice_array, curr_sum, turn + 1, desired_total, used);
                force_win &= res;
                *curr_sum -= choice_array[i];
                used[choice_array[i] as usize] = false;
            }

            force_win
        }

        if desired_total <= 0 || desired_total <= max_choosable_integer {
            return true;
        }
        let mut max_sum = (1 + max_choosable_integer) * max_choosable_integer / 2;
        if max_sum < desired_total {
            return false;
        }

        let choice_array = (1..=max_choosable_integer).collect::<Vec<i32>>();
        let mut curr_sum = 0;
        let mut turn = 0;
        let mut used = vec![false; max_choosable_integer as usize + 1];
        let mut desired_total = desired_total;
        backtrack(
            &choice_array,
            &mut curr_sum,
            turn,
            &mut desired_total,
            &mut used,
        )
    }
}
