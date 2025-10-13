struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        fn is_anagram(s1: &str, s2: &str) -> bool {
            let mut count = [0; 26];
            for b in s1.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            for b in s2.bytes() {
                count[(b - b'a') as usize] -= 1;
            }
            count.iter().all(|&c| c == 0)
        }

        let mut ans = Vec::new();
        ans.push(words[0].clone());
        for i in 1..words.len() {
            if !is_anagram(&words[i], &words[i - 1]) {
                ans.push(words[i].clone());
            }
        }

        ans
    }
}
