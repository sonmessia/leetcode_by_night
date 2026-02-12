struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();

        let mut ans = 0;

        for i in 0..n {
            let mut cnts = vec![0; 26];

            let (mut all_cnt, mut max_cnt, mut max_freq) = (0, 0, 0);

            for j in i..n {
                let idx = (s[j] - b'a') as usize;
                cnts[idx] += 1;

                if cnts[idx] == 1 {
                    all_cnt += 1;
                }

                if cnts[idx] == max_freq {
                    max_cnt += 1;
                }

                if cnts[idx] > max_freq {
                    max_freq = cnts[idx];
                    max_cnt = 1;
                }

                if all_cnt == max_cnt {
                    ans = ans.max((j - i + 1) as i32);
                }
            }
        }

        ans
    }
}
