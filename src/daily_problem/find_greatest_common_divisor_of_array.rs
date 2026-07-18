struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let gcd = |mut a: i32, mut b: i32| {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        };

        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        gcd(*min, *max)
    }
}
