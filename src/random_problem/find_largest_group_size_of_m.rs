struct Solution;

impl Solution {
    pub fn largest_group_size_of_m(arr: Vec<i32>, m: i32) -> i32 {
        let n = arr.len();
        let mut ans = -1;

        let (mut length, mut count) = (vec![0; n + 2], vec![0; n + 1]);

        for i in 0..n {
            let curr = arr[i] as usize;
            let left = length[curr - 1];
            let right = length[curr + 1];

            length[curr - left] = left + right + 1;
            length[curr] = left + right + 1;
            length[curr + right] = left + right + 1;

            count[left] -= 1;
            count[right] -= 1;
            count[length[curr]] += 1;

            if count[m as usize] > 0 {
                ans = i as i32 + 1;
            }
        }
        ans
    }
}
