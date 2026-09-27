struct Solution;

impl Solution {
    fn build_lps(pattern: &str) -> Vec<usize> {
        let m = pattern.len();
        let mut lps = vec![0; m];
        let mut length = 0;
        let mut i = 1;

        while i < m {
            if pattern.as_bytes()[i] == pattern.as_bytes()[length] {
                length += 1;
                lps[i] = length;
                i += 1;
            } else {
                if length != 0 {
                    length = lps[length - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }

        lps
    }

    fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
        let n = text.len();
        let m = pattern.len();
        let lps = Self::build_lps(pattern);
        let mut result = Vec::new();

        let mut i = 0; // index for text
        let mut j = 0; // index for pattern

        while i < n {
            if pattern.as_bytes()[j] == text.as_bytes()[i] {
                i += 1;
                j += 1;
            }

            if j == m {
                result.push(i - j);
                j = lps[j - 1];
            } else if i < n && pattern.as_bytes()[j] != text.as_bytes()[i] {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }

        result
    }

    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        let mut result = Vec::new();

        let a_indices = Self::kmp_search(&s, &a);
        let b_indices = Self::kmp_search(&s, &b);

        let n = b_indices.len();
        let mut j = 0;

        for &ai in &a_indices {
            while j < n && (b_indices[j] + k as usize) < ai {
                j += 1;
            }

            if j < n && b_indices[j] <= ai + k as usize {
                result.push(ai as i32);
            }
        }

        result
    }
}
