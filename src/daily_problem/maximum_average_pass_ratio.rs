struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct State {
    gain: f64,
    pass: i32,
    total: i32,
}

impl State {
    fn new(pass: i32, total: i32) -> Self {
        Self {
            gain: State::calc_gain(pass, total),
            pass,
            total,
        }
    }

    fn calc_gain(pass: i32, total: i32) -> f64 {
        (pass as f64 + 1.0) / (total as f64 + 1.0) - (pass as f64) / (total as f64)
    }
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.gain == other.gain
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gain
            .partial_cmp(&other.gain)
            .unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = BinaryHeap::new();
        let mut sum = 0.0;

        for c in classes.iter() {
            let pass = c[0];
            let total = c[1];
            sum += pass as f64 / total as f64;
            heap.push(State::new(pass, total));
        }

        for _ in 0..extra_students {
            if let Some(mut state) = heap.pop() {
                sum -= state.pass as f64 / state.total as f64;
                state.pass += 1;
                state.total += 1;
                sum += state.pass as f64 / state.total as f64;
                state.gain = State::calc_gain(state.pass, state.total);
                heap.push(state);
            }
        }

        sum / classes.len() as f64
    }
}

fn main() {
    let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
    let extra_students = 2;
    let result = Solution::max_average_ratio(classes, extra_students);
    println!("{:.5}", result); // Output: 0.78333
}
