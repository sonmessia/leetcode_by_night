struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut total = apple.iter().sum::<i32>();

        capacity.sort_unstable_by(|a, b| b.cmp(a));

        for i in capacity {
            if total > 0 {
                total -= i;
                ans += 1;
            }
        }
        ans
    }
}
