struct Solution;

impl Solution {
    pub fn lex_smallest_negated_perm(n: i32, mut target: i64) -> Vec<i32> {
        let mut total_sum: i64 = (n as i64 * (n as i64 + 1)) / 2;
        if target > total_sum || target < -total_sum || (total_sum - target) % 2 != 0 {
            return vec![];
        }

        let mut v = Vec::with_capacity(n as usize);

        for i in (1..=n).rev() {
            let var = i as i64;
            total_sum -= var;

            if total_sum - var >= target {
                target += var;
                v.push(-i);
            } else {
                target -= var;
                v.push(i);
            }
            println!("{}", target);
        }

        if target != 0 {
            return vec![];
        }
        v.sort_unstable();
        v
    }
}

fn main() {
    let v = Solution::lex_smallest_negated_perm(6, 7);
    println!("{:?}", v);
}
