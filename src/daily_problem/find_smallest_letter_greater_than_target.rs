struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let n = letters.len();
        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            let mid = (left + (right - left) / 2) as usize;
            if letters[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if letters[left] > target {
            letters[left]
        } else {
            letters[0]
        }
    }
}
