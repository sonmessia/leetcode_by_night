struct Solution;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 10],
    is_end_of_number: bool,
}

struct Trie {
    root: TrieNode,
}

impl Trie {}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut ans = 0;

        ans
    }
}
