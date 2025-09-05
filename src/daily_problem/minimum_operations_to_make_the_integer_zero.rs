struct Solution;

impl Solution {
    pub fn min_operations(num1: i32, num2: i32) -> i32 {
        for k in 1..=60 {
            let mut x: i64 = num1 as i64 - k * num2 as i64;
            if x < k {
                return -1;
            }
            if k >= x.count_ones() as i64 {
                return k as i32;
            }
        }
        -1
   }
}

