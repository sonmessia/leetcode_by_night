struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; a.len()];
        let mut count = vec![0; a.len() + 1];
        for i in 0..a.len() {
            count[a[i] as usize] += 1;
            count[b[i] as usize] += 1;
            ans[i] = ans[i.saturating_sub(1)]
                + if count[a[i] as usize] == 2 { 1 } else { 0 }
                + if a[i] != b[i] && count[b[i] as usize] == 2 {
                    1
                } else {
                    0
                };
        }
        ans
    }
}
