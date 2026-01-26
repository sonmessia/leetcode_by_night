struct Solution;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        let mut result = Vec::new();
        arr.sort_unstable();
        let min_diff = arr.windows(2).map(|w| w[1] - w[0]).min().unwrap();

        for i in 0..arr.len() - 1 {
            if arr[i + 1] - arr[i] == min_diff {
                result.push(vec![arr[i], arr[i + 1]]);
            }
        }

        result
    }
}
