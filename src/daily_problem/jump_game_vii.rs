struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let mut reachable = vec![false; n];
        reachable[0] = true;
        let mut farthest = 0;
        for i in 0..n {
            if !reachable[i] || s[i] == b'1' {
                continue;
            }
            if i + min_jump as usize >= n {
                break;
            }
            farthest = farthest.max(i + min_jump as usize);
            for j in farthest..=(i + max_jump as usize).min(n - 1) {
                if s[j] == b'0' {
                    reachable[j] = true;
                }
            }
            farthest = (i + max_jump as usize).min(n - 1);
        }
        reachable[n - 1]
    }
}
