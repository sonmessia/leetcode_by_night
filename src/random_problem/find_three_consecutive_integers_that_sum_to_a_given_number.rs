struct Solution;

impl Solution {
    pub fn sum_of_three(num: i32) -> Vec<i32> {
        if num % 3 == 0 {
            let a = num / 3;
            vec![a - 1, a, a + 1]
        } else {
            vec![]
        }
    }
}
