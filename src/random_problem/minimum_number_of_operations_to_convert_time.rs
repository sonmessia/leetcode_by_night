struct Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let h = &current[..2].parse::<i32>().map_or(0, |x| x);
        let m = &current[3..].parse::<i32>().unwrap_or(0);
        let start = h * 60 + m;

        let h = &correct[..2].parse::<i32>().map_or(0, |x| x);
        let m = &correct[3..].parse::<i32>().unwrap_or(0);
        let end = h * 60 + m;
        let mut diff = end - start;

        let mut ans = diff / 60;
        diff %= 60;
        ans += diff / 15;
        diff %= 15;
        ans += diff / 5;
        diff %= 5;
        ans += diff;

        ans
    }
}
