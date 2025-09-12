struct Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.chars()
            .any(|x| x == 'a' || x == 'e' || x == 'i' || x == 'o' || x == 'u')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_does_alice_win() {
        assert_eq!(Solution::does_alice_win("a".to_string()), true);
        assert_eq!(Solution::does_alice_win("bbcd".to_string()), false);
        assert_eq!(Solution::does_alice_win("leetcode".to_string()), true);
        assert_eq!(Solution::does_alice_win("rhythm".to_string()), false);
    }
}
