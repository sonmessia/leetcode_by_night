use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        if k == 1 {
            return nums;
        }
        let mut deq: VecDeque<usize> = VecDeque::new();
        let mut max_idx = 0;
        for i in 0..k {
            while !deq.is_empty() && nums[i] > nums[*deq.back().unwrap()] {
                deq.pop_back();
            }
            deq.push_back(i);
            if nums[i] > nums[max_idx] {
                max_idx = i;
            }
        }
        println!("{:?}", deq);
        let mut output = vec![nums[max_idx]];
        for i in k..n {
            while !deq.is_empty() && deq[0] <= i - k {
                deq.pop_front();
            }
            while !deq.is_empty() && nums[i] > nums[*deq.back().unwrap()] {
                deq.pop_back();
            }
            deq.push_back(i);
            output.push(nums[*deq.front().unwrap()]);
            println!("loop {:?}", deq);
        }
        output
    }
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let result = Solution::max_sliding_window(nums, k);
    println!("{:?}", result); // Output: [3, 3, 5, 5, 6, 7]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
        let nums = vec![1];
        let k = 1;
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, vec![1]);
        let nums = vec![1, -1];
        let k = 1;
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, vec![1, -1]);
        let nums = vec![9, 11];
        let k = 2;
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, vec![11]);
        let nums = vec![4, -2];
        let k = 2;
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, vec![4]);
    }
}
