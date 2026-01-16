struct Solution;

impl Solution {
    pub fn max_strength(mut nums: Vec<i32>) -> i64 {
        let mut positive = vec![];
        let mut negative = vec![];

        for &num in &nums {
            if num > 0 {
                positive.push(num as i64);
            } else if num < 0 {
                negative.push(num as i64);
            }
        }

        nums.sort_unstable();
        negative.sort_unstable();

        if negative.len() % 2 != 0 {
            negative.pop();
        }

        if positive.is_empty() && negative.is_empty() {
            return *nums.iter().max().unwrap() as i64;
        }

        let mut product: i64 = 1;

        for p in positive {
            product *= p;
        }

        for ne in negative {
            product *= ne;
        }
        product
    }
}
