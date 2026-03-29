struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        let (mut odd_cnt, mut even_cnt) = ([0; 26], [0; 26]);

        for i in 0..s1.len() {
            if i % 2 == 0 {
                even_cnt[(s1[i] - b'a') as usize] += 1;
                even_cnt[(s2[i] - b'a') as usize] -= 1;
            } else {
                odd_cnt[(s1[i] - b'a') as usize] += 1;
                odd_cnt[(s2[i] - b'a') as usize] -= 1;
            }
        }

        odd_cnt.iter().all(|&x| x == 0) && even_cnt.iter().all(|&x| x == 0)
    }
}
