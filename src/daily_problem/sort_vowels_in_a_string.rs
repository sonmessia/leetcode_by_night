struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut vowels: Vec<char> = chars
            .iter()
            .filter(|&&c| "aeiouAEIOU".contains(c))
            .cloned()
            .collect();
        vowels.sort_unstable_by(|a, b| a.cmp(&b));
        println!("{:?}", vowels);
        let mut vowel_index = 0;
        for i in 0..chars.len() {
            if "aeiouAEIOU".contains(chars[i]) {
                chars[i] = vowels[vowel_index];
                vowel_index += 1;
            }
        }
        chars.into_iter().collect()
    }
}

fn main() {
    let s = "lEetcOde".to_string();
    let result = Solution::sort_vowels(s);
    println!("{}", result); // Output: "lEOtcede"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_vowels() {
        let s = "lEetcOde".to_string();
        let result = Solution::sort_vowels(s);
        assert_eq!(result, "lEOtcede");
        let s = "LeetCode".to_string();
        let result = Solution::sort_vowels(s);
        assert_eq!(result, "LeetCedo");
        let s = "aeiou".to_string();
        let result = Solution::sort_vowels(s);
        assert_eq!(result, "aeiou");
        let s = "leetcode".to_string();
        let result = Solution::sort_vowels(s);
        assert_eq!(result, "leetcedo");
    }
}
