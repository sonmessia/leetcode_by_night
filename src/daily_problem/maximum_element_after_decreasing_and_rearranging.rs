struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        arr[0] = 1;
        for i in 0..arr.len() - 1 {
            if arr[i + 1] - arr[i] > 1 {
                arr[i + 1] = arr[i] + 1;
            }
        }

        arr[arr.len() - 1]
    }
}
