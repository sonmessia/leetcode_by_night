struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut chars: Vec<char> = s.chars().collect();
        let mut ans = 0;

        let mut count_ones = 0;
        let mut count_zeros = 0;

        let n = chars.len();
        for i in 0..n {
            if chars[i] == '0' {
                count_zeros += 1;
            }

            if chars[i] == '1' {
                count_ones += 1;
                if count_ones >= 2 && count_zeros > 0 {
                    ans += count_ones - 1;
                    count_zeros = 0;
                } else {
                    count_zeros = 0;
                }
            }

            // println!(
            //     "i: {}, count_ones: {}, count_zeros: {}, ans: {}",
            //     i, count_ones, count_zeros, ans
            // );
        }
        if chars[n - 1] == '0' {
            ans += count_ones;
        }
        ans
    }
}

fn main() {
    let s = "1001101".to_string();
    let result = Solution::max_operations(s);
    println!("Maximum number of operations: {}", result); // Output: 4
}
