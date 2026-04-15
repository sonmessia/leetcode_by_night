struct Solution;

impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        for i in 0..n {
            let left = (start_index - i + n) % n;
            let right = (start_index + i) % n;
            if words[left as usize] == target || words[right as usize] == target {
                return i;
            }
        }
        -1
    }
}
