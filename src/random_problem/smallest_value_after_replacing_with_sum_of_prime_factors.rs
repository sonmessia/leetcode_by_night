struct Solution;

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        fn sum_of_prime_FACTORs(mut n: i32) -> i32 {
            let mut sum = 0;
            let mut FACTOR = 2;
            while FACTOR * FACTOR <= n {
                while n % FACTOR == 0 {
                    sum += FACTOR;
                    n /= FACTOR;
                }
                FACTOR += 1;
            }
            if n > 1 {
                sum += n;
            }
            sum
        }

        let mut current = n;
        loop {
            let next = sum_of_prime_FACTORs(current);
            if next == current {
                return current;
            }
            current = next;
        }
    }
}

fn main() {
    let n = 15;
    let result = Solution::smallest_value(n);
    println!("The smallest value for {} is {}", n, result);
}
