struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        // let mut left = vec![i32::MAX; n];
        let mut right = vec![i32::MAX; n];
        let mut sum = 0;
        let mut start = 0;

        // for end in 0..n {
        //     sum += arr[end];
        //     while sum > target {
        //         sum -= arr[start];
        //         start += 1;
        //     }
        //     if sum == target {
        //         left[end] = (end - start + 1) as i32;
        //     }
        //     if end > 0 {
        //         left[end] = left[end].min(left[end - 1]);
        //     }
        // }

        let mut end = n - 1;

        for start in (0..n).rev() {
            sum += arr[start];
            while sum > target {
                sum -= arr[end];
                end -= 1;
            }
            if sum == target {
                right[start] = (end - start + 1) as i32;
            }

            if start < n - 1 {
                right[start] = right[start].min(right[start + 1]);
            }
        }

        // println!("left: {:?}", left);
        // println!("right: {:?}", right);

        let mut result = i32::MAX;

        sum = 0;
        for end in 0..n - 1 {
            sum += arr[end];
            while sum > target {
                sum -= arr[start];
                start += 1;
            }

            if sum == target {
                let left_len = (end - start + 1) as i32;
                if end + 1 < n && right[end + 1] != i32::MAX {
                    result = result.min(left_len + right[end + 1]);
                }
            }
        }
        if result == i32::MAX { -1 } else { result }
    }
}
