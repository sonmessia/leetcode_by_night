struct Solution;

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_by(|a, b| a[1].cmp(&b[1]));
        println!("{:?}", pairs);
        let mut count = 0;
        let mut current_end = i32::MIN;
        for pair in pairs {
            if pair[0] > current_end {
                count += 1;
                current_end = pair[1];
            }
        }
        count
    }
}

fn main() {
    let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    let result = Solution::find_longest_chain(pairs);
    println!("{}", result); // Output: 2
}
