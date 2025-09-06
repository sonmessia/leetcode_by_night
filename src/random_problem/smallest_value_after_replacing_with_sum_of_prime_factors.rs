struct Solution;

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        fn sum_of_prime_factors(mut n: i32) -> i32 {
            let mut sum = 0;
            let mut factor = 2;
            while factor * factor <= n {
                while n % factor == 0 {
                    sum += factor;
                    n /= factor;
                }
                factor += 1;
            }
            if n > 1 {
                sum += n;
            }
            sum
        }

        let mut current = n;
        loop {
            let next = sum_of_prime_factors(current);
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
