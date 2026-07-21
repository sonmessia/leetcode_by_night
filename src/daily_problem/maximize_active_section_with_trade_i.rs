struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let mut ans = 0;

        let (mut one_cnts, mut zero_cnts) = (0, 0);
        let mut tmp = vec![];

        for ch in s.chars() {
            if ch == '1' {
                one_cnts += 1;
                if zero_cnts > 0 {
                    tmp.push(zero_cnts);
                    zero_cnts = 0;
                }
            } else {
                zero_cnts += 1;
            }
        }

        if zero_cnts > 0 {
            tmp.push(zero_cnts);
        }

        println!("tmp: {:?}", tmp);

        for i in 1..tmp.len() {
            ans = ans.max(tmp[i - 1] + tmp[i]);
        }

        ans += one_cnts;
        ans
    }
}
