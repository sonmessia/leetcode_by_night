#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_number: bool,
}
struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr_node = &mut self.root;

        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;

            if curr_node.children[index].is_none() {
                curr_node.children[index] = Some(Box::new(TrieNode::default()));
            }

            curr_node = curr_node.children[index].as_mut().unwrap();
        }

        curr_node.is_end_of_number = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr_node = &self.root;

        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;

            if curr_node.children[index].is_none() {
                return false;
            }

            curr_node = curr_node.children[index].as_ref().unwrap();
        }

        curr_node.is_end_of_number
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr_node = &self.root;

        for c in prefix.chars() {
            let index = (c as u8 - b'a') as usize;

            if curr_node.children[index].is_none() {
                return false;
            }

            curr_node = curr_node.children[index].as_ref().unwrap();
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {
    let mut obj = Trie::new();
    obj.insert("word".to_string());
    let ret_2: bool = obj.search("word".to_string());
    let ret_3: bool = obj.starts_with("wor".to_string());
}
