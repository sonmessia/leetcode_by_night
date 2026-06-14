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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        let mut stack = Vec::new();

        let mut ans = 0;

        while let Some(node) = slow {
            stack.push(node.val);
            slow = node.next.as_ref();
        }

        while let Some(node) = fast {
            let twin = stack.pop().unwrap();
            ans = ans.max(node.val + twin);
            fast = node.next.as_ref();
        }

        ans
    }
}
