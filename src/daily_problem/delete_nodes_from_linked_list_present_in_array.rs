// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn delete_nodes(head: Option<Box<ListNode>>, arr: Vec<i32>) -> Option<Box<ListNode>> {
        use std::collections::HashSet;
        let to_delete: HashSet<i32> = arr.into_iter().collect();
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut node = head;

        while let Some(mut boxed_node) = node {
            if !to_delete.contains(&boxed_node.val) {
                current.next = Some(Box::new(ListNode::new(boxed_node.val)));
                current = current.next.as_mut().unwrap();
            }
            node = boxed_node.next;
        }

        dummy.next
    }
}
