struct Soluton;

impl Soluton {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key.as_bytes();
        let (mut left, mut right) = (0, 0);
        let (mut count_t, mut count_f) = (0, 0);
        let mut max_len = 0;
        let k = k as usize;
        while right < answer_key.len() {
            if answer_key[right] == b'T' {
                count_t += 1;
            } else {
                count_f += 1;
            }
            while count_t > k && count_f > k {
                if answer_key[left] == b'T' {
                    count_t -= 1;
                } else {
                    count_f -= 1;
                }
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
            right += 1;
        }
        max_len as i32
    }
}
