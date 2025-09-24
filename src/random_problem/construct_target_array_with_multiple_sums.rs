struct Solution;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        let mut total: i64 = 0;
        for &num in &target {
            heap.push(num as i64);
            total += num as i64;
        }
        while let Some(largest) = heap.pop() {
            let rest = total - largest;
            if largest == 1 || rest == 1 {
                return true;
            }
            if rest == 0 || largest < rest || largest % rest == 0 {
                return false;
            }
            let new_value = largest % rest;
            total = rest + new_value;
            heap.push(new_value);
        }

        true
    }
}
