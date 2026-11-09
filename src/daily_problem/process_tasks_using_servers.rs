use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; tasks.len()];

        let mut available = BinaryHeap::new();

        let mut busy = BinaryHeap::new();

        for (i, &weight) in servers.iter().enumerate() {
            available.push(Reverse((weight, i)));
        }

        let mut time = 0_i64;

        for (i, &duration) in tasks.iter().enumerate() {
            time = time.max(i as i64);

            while let Some(&Reverse((finish, weight, index))) = busy.peek() {
                if finish <= time {
                    busy.pop();
                    available.push(Reverse((weight, index)));
                } else {
                    break;
                }
            }

            if available.is_empty() {
                let Reverse((finish, _, _)) = *busy.peek().unwrap();
                time = finish;

                while let Some(&Reverse((finish, weight, index))) = busy.peek() {
                    if finish <= time {
                        busy.pop();
                        available.push(Reverse((weight, index)));
                    } else {
                        break;
                    }
                }
            }

            let Reverse((weight, index)) = available.pop().unwrap();

            result[i] = index as i32;

            busy.push(Reverse((time + duration as i64, weight, index)));
        }

        result
    }
}
