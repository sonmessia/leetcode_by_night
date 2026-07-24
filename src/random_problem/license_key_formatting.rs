struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut cnt = 0;
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();

        let mut ans = String::new();

        for i in (0..n).rev() {
            if s[i] != '-' {
                ans.push(s[i].to_ascii_uppercase());
                cnt += 1;

                if cnt == k as usize {
                    ans.push('-');
                    cnt = 0;
                }
            }
        }

        if ans.ends_with('-') {
            ans.pop();
        }

        ans = ans.chars().rev().collect();
        ans
    }
}
