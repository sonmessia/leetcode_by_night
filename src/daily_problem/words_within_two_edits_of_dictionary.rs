struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ans = vec![];
        for q in queries {
            for d in &dictionary {
                let mut diff = 0;
                for (a, b) in q.chars().zip(d.chars()) {
                    if a != b {
                        diff += 1;
                    }
                }
                if diff <= 2 {
                    ans.push(q);
                    break;
                }
            }
        }
        ans
    }
}
