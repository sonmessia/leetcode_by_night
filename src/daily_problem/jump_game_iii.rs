struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut visited = vec![false; n];
        let mut stack = vec![start as usize];
        while let Some(i) = stack.pop() {
            if arr[i] == 0 {
                return true;
            }
            if visited[i] {
                continue;
            }
            visited[i] = true;
            if i + (arr[i] as usize) < n {
                stack.push(i + arr[i] as usize);
            }
            if i >= arr[i] as usize {
                stack.push(i - arr[i] as usize);
            }
        }
        false
    }
}
