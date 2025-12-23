struct Solution;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut events = events;
        events.sort_unstable_by_key(|e| e[0]);

        let (mut max_val, mut max_sum) = (0, 0);
        let mut pq = BinaryHeap::new();

        for event in events {
            let start = event[0];
            let end = event[1];
            let val = event[2];

            while let Some(Reverse((e_end, e_val))) = pq.peek() {
                if *e_end < start {
                    max_val = max_val.max(*e_val);
                    pq.pop();
                } else {
                    break;
                }
            }

            max_sum = max_sum.max(max_val + val);

            pq.push(Reverse((end, val)));
        }

        max_sum
    }
}
