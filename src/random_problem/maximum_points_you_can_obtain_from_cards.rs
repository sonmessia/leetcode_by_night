struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let mut sum = 0;
        for i in 0..k {
            sum += card_points[i];
        }
        let mut ans = sum;
        for i in 0..k {
            sum -= card_points[k - 1 - i];
            sum += card_points[n - 1 - i];
            ans = ans.max(sum);
        }
        ans
    }
}
