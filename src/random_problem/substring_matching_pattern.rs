struct Solution;

impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        let (left, right) = p.split_once('*').unwrap();

        match (s.find(left), s.rfind(right)) {
            (Some(l), Some(r)) if l + left.len() <= r => true,
            _ => false,
        }
    }
}
