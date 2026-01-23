use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut array: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let mut flipped: i64 = 0;

        let mut left: Vec<Option<usize>> = (0..n)
            .map(|i| if i > 0 { Some(i - 1) } else { None })
            .collect();
        let mut right: Vec<Option<usize>> = (0..n)
            .map(|i| if i < n - 1 { Some(i + 1) } else { None })
            .collect();

        let mut pair_sum: BTreeSet<(i64, usize)> = BTreeSet::new();

        for i in 0..n - 1 {
            if array[i] > array[i + 1] {
                flipped += 1;
            }
            pair_sum.insert((array[i] + array[i + 1], i));
        }

        let mut op = 0;

        while flipped > 0 {
            let (sum, i) = match pair_sum.iter().next() {
                Some(&val) => val,
                None => break,
            };
            pair_sum.remove(&(sum, i));

            let j = right[i].expect("Right index must exist");
            let h = left[i];
            let k = right[j];

            Self::update_state(h, &array, &right, &mut pair_sum, &mut flipped, false);

            if array[i] > array[j] {
                flipped -= 1;
            }

            Self::update_state(Some(j), &array, &right, &mut pair_sum, &mut flipped, false);

            array[i] += array[j];
            op += 1;

            right[i] = k;
            if let Some(next_idx) = k {
                left[next_idx] = Some(i);
            }

            Self::update_state(h, &array, &right, &mut pair_sum, &mut flipped, true);
            Self::update_state(Some(i), &array, &right, &mut pair_sum, &mut flipped, true);
        }

        op
    }

    fn update_state(
        idx_opt: Option<usize>,
        array: &[i64],
        right: &[Option<usize>],
        pair_sum: &mut BTreeSet<(i64, usize)>,
        flipped: &mut i64,
        is_add: bool,
    ) {
        if let Some(i) = idx_opt {
            if let Some(j) = right[i] {
                let current_sum = array[i] + array[j];
                let is_flipped = array[i] > array[j];

                if is_add {
                    if pair_sum.insert((current_sum, i)) && is_flipped {
                        *flipped += 1;
                    }
                } else {
                    if pair_sum.remove(&(current_sum, i)) && is_flipped {
                        *flipped -= 1;
                    }
                }
            }
        }
    }
}
