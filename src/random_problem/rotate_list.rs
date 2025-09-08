struct Solution;

struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        if k == 0 {
            return head;
        }

        let mut len = 0;
        {
            let mut node_ref = head.as_ref();
            while let Some(node) = node_ref {
                len += 1;
                node_ref = node.next.as_ref();
            }
        }

        if len == 0 {
            return head;
        }
        let k = k % len;
        if k == 0 {
            return head;
        }
        let mut node_ref = head.as_deref_mut().unwrap();
        for _ in 0..(len - k - 1) {
            node_ref = node_ref.next.as_deref_mut().unwrap();
        }
        let mut new_head = node_ref.next.take().unwrap();
        node_ref = new_head.as_mut();
        while node_ref.next.is_some() {
            node_ref = node_ref.next.as_deref_mut().unwrap();
        }
        node_ref.next = head;
        Some(new_head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }
    fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }
    #[test]
    fn test_rotate_right() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        let k = 2;
        let result = Solution::rotate_right(head, k);
        assert_eq!(to_vec(result), vec![4, 5, 1, 2, 3]);
        let head = to_list(vec![0, 1, 2]);
        let k = 4;
        let result = Solution::rotate_right(head, k);
        assert_eq!(to_vec(result), vec![2, 0, 1]);
        let head = to_list(vec![]);
        let k = 0;
        let result = Solution::rotate_right(head, k);
        assert_eq!(to_vec(result), vec![]);
        let head = to_list(vec![1]);
        let k = 99;
        let result = Solution::rotate_right(head, k);
        assert_eq!(to_vec(result), vec![1]);
    }
}
