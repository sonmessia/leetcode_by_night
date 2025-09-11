#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut res = Vec::new();
        for size in (1..=arr.len()).rev() {
            let max_index = (0..size).max_by_key(|&i| arr[i]).unwrap();
            println!("size: {size}, max_index: {max_index}");
            if max_index + 1 != size {
                if max_index != 0 {
                    arr[..=max_index].reverse();
                    res.push((max_index + 1) as i32);
                }
                arr[..size].reverse();
                res.push(size as i32);
            }
        }
        res
    }
}

#[allow(dead_code)]
fn main() {
    let arr = vec![3, 2, 4, 1];
    let result = Solution::pancake_sort(arr);
    println!("{:?}", result); // Output: [3, 4, 2, 3, 2]
}
