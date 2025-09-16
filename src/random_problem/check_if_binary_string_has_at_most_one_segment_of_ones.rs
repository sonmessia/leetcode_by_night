struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let (mut left, mut right) = (0, n - 1);
        while left < n {
            if chars[left] == '1' {
                break;
            }
            left += 1;
        }

        while right as i32 >= 0 {
            if chars[right] == '1' {
                break;
            }
            right -= 1;
        }

        if left == right {
            return true;
        }

        for i in (left + 1)..right {
            if chars[i] == '0' {
                return false;
            }
        }
        true
    }
}
