struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }

        let mut is_prime = vec![true; (n as usize)];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..((n as f64).sqrt() as usize + 1) {
            if is_prime[i] {
                let mut j = i * i;
                while j < is_prime.len() {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }

        is_prime.iter().filter(|&&x| x).count() as i32
    }
}
