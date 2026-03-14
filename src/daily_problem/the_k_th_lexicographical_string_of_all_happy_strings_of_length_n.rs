struct Solution;

impl Solution {
    fn gen_happy_string(s: String, n: i32, happy_string: &mut Vec<String>) {
        if n == 0 {
            happy_string.push(s);
            return;
        }

        for ch in ['a', 'b', 'c'] {
            if s.is_empty() || s.chars().last().unwrap() != ch {
                let mut new_s = s.clone();
                new_s.push(ch);
                Self::gen_happy_string(new_s, n - 1, happy_string);
            }
        }
    }
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut ans: Vec<String> = vec![];

        Self::gen_happy_string(String::new(), n, &mut ans);
        if k as usize > ans.len() {
            return String::new();
        }

        ans[k as usize - 1].clone()
    }
}
