struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut total = num_bottles;
        let mut empty_bottles = num_bottles;
        while empty_bottles >= num_exchange {
            let new_bottles = empty_bottles / num_exchange;
            total += new_bottles;
            empty_bottles = empty_bottles % num_exchange + new_bottles;
        }
        total
    }
}
