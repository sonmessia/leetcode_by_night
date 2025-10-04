struct Solution;

impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;
        let len: usize = height.len();
        if len == 0 {
            return 0;
        }
        let mut l: usize = 0;
        let mut r: usize = len - 1;
        let mut result: usize = 0;
        while l < r {
            let min_height: usize = cmp::min(height[l] as usize, height[r] as usize);
            result = cmp::max(result, (r - l) * min_height);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        result as i32
    }
}
