struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();

        if bytes.iter().filter(|&&c| c == b'1').count() < k as usize {
            return String::new();
        }

        let mut left = 0;
        let mut count = 0;

        let mut best_left = 0;
        let mut best_right = bytes.len();

        for right in 0..bytes.len() {
            count += (bytes[right] - b'0') as i32;

            while count > k || bytes[left] == b'0' {
                count -= (bytes[left] - b'0') as i32;
                left += 1;
            }

            if count == k {
                let current_len = right - left + 1;
                let best_len = best_right - best_left;

                if current_len < best_len
                    || (current_len == best_len
                        && bytes[left..=right] < bytes[best_left..best_right])
                {
                    best_left = left;
                    best_right = right + 1;
                }
            }
        }

        String::from_utf8(bytes[best_left..best_right].to_vec()).unwrap()
    }
}
