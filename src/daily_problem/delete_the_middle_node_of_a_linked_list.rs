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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref()?.next.is_none() {
            return None;
        }

        let mut len = 0;
        let mut curr = head.as_ref();

        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }

        let middle = len / 2;

        let mut curr = head.as_mut().unwrap();

        for _ in 0..middle - 1 {
            curr = curr.next.as_mut().unwrap();
        }

        let next = curr.next.as_mut().unwrap().next.take();
        curr.next = next;

        head
    }
}
