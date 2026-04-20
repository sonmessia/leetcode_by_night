struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        if colors[0] != colors[n - 1] {
            return (n - 1) as i32;
        }

        let mut ans = 0;

        for (i, color) in colors.iter().enumerate() {
            if *color != colors[0] {
                ans = ans.max(i.max(n - i - 1) as i32);
            }
        }

        ans
    }
}
