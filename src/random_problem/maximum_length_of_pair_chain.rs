struct Solution;

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_by(|a, b| a[1].cmp(&b[1]));
        println!("{:?}", pairs);
        let mut cnt = 0;
        let mut curr_end = i32::MIN;
        for pair in pairs {
            if pair[0] > curr_end {
                cnt += 1;
                curr_end = pair[1];
            }
        }
        cnt
    }
}

fn main() {
    let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    let result = Solution::find_longest_chain(pairs);
    println!("{}", result); // Output: 2
}
