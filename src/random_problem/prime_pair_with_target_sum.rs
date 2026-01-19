struct Solution;

impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        let mut is_prime = vec![true; n as usize + 5];

        fn sieve(is_prime: &mut Vec<bool>) {
            is_prime[0] = false;
            is_prime[1] = false;

            for i in 2..is_prime.len() {
                if is_prime[i] {
                    let mut j = i * 2;
                    while j < is_prime.len() {
                        is_prime[j] = false;
                        j += i;
                    }
                }
            }
        }

        sieve(&mut is_prime);

        let mut ans = vec![];
        for i in 1..=n / 2 {
            if is_prime[i as usize] && is_prime[(n - i) as usize] {
                ans.push(vec![i, n - i]);
            }
        }

        ans
    }
}
