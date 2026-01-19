struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let target = target as u64;

        for (i, v) in variables.iter().enumerate() {
            let a = v[0] as u64;
            let b = v[1] as u64;
            let c = v[2] as u64;
            let m = v[3] as u64;

            let part1 = Self::mod_pow(a, b, 10);

            let part2 = Self::mod_pow(part1, c, m);

            if part2 == target {
                result.push(i as i32);
            }
        }

        result
    }

    fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        if modulus == 1 {
            return 0;
        }
        let mut res = 1;
        base %= modulus;

        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % modulus;
            }
            base = (base * base) % modulus;
            exp /= 2;
        }
        res
    }
}
