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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut head = head;

        let mut prev_node = if let Some(h) = head {
            head = h.next;
            h.val
        } else {
            0
        };

        let mut current_node = if let Some(h) = head {
            head = h.next;
            h.val
        } else {
            0
        };

        let mut idx = 1;

        let mut critical_points = Vec::new();

        while let Some(node) = head {
            if (prev_node < current_node && current_node > node.val)
                || (prev_node > current_node && current_node < node.val)
            {
                critical_points.push(idx);
            }

            prev_node = current_node;
            current_node = node.val;
            head = node.next;
            idx += 1;
        }

        if critical_points.len() < 2 {
            return vec![-1, -1];
        } else {
            vec![
                critical_points
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .min()
                    .unwrap_or(i32::MAX) as i32,
                critical_points.last().unwrap() - critical_points.first().unwrap(),
            ]
        }
    }
}
