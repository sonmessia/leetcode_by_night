struct Solution;

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    index: usize,
    min_length: usize,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            index: usize::MAX,
            min_length: usize::MAX,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str, index: usize) {
        let mut node = &mut self.root;
        let length = word.len();
        let s = word.as_bytes();
        if length < node.min_length || (length == node.min_length && index < node.index) {
            node.min_length = length;
            node.index = index;
        }

        for i in (0..length).rev() {
            let idx = (s[i] - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
            if length < node.min_length || (length == node.min_length && index < node.index) {
                node.min_length = length;
                node.index = index;
            }
        }
    }

    fn query(&self, word: &str) -> i32 {
        let s = word.as_bytes();
        let length = s.len();
        let mut node = &self.root;
        for i in (0..length).rev() {
            let idx = (s[i] - b'a') as usize;
            if let Some(child) = &node.children[idx] {
                node = child;
            } else {
                break;
            }
        }
        node.index as i32
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();

        for (i, word) in words_container.iter().enumerate() {
            trie.insert(word, i);
        }

        words_query.iter().map(|query| trie.query(query)).collect()
    }
}
