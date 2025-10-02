struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut empty_bottles = num_bottles;

        while empty_bottles >= num_exchange {
            let mut exchanged = 0;
            while empty_bottles >= num_exchange {
                empty_bottles -= num_exchange;
                exchanged += 1;
                num_exchange += 1;
            }
            ans += exchanged;
            empty_bottles += exchanged;
        }
        ans
    }
}

fn main() {
    let num_bottles = 15;
    let num_exchange = 6;
    let result = Solution::max_bottles_drunk(num_bottles, num_exchange);
    println!("Max bottles drunk: {}", result);

    let num_bottles = 10;
    let num_exchange = 3;
    let result = Solution::max_bottles_drunk(num_bottles, num_exchange);
    println!("Max bottles drunk: {}", result);
}
