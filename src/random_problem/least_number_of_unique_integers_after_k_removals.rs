struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        use std::collections::{BinaryHeap, HashMap};

        let mut freq = HashMap::new();
        let mut pq = BinaryHeap::new();
        let n = arr.len();
        for &num in &arr {
            *freq.entry(num).or_insert(0) += 1;
        }

        for v in freq.values() {
            pq.push(-*v);
        }

        let mut elems_removed = 0;

        while let Some(count) = pq.pop() {
            elems_removed += -count;
            if elems_removed > k {
                return pq.len() as i32 + 1;
            }
        }
        return 0;
    }
}

fn main() {
    let arr = vec![4, 3, 1, 1, 3, 3, 2];
    let k = 3;
    let result = Solution::find_least_num_of_unique_ints(arr, k);
    println!("Result: {}", result); // Output: 1
}
