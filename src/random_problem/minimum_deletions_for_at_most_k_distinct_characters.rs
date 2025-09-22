struct Solution;

impl Solution {
    pub fn min_deletions(s: String, k: i32) -> i32 {
        let mut freq = [0; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        freq.sort_unstable_by(|a, b| b.cmp(a));
        println!("{:?}", freq);
        let mut deletions = 0;
        for &f in freq.iter().skip(k as usize) {
            println!("f: {}", f);
            deletions += f;
        }
        deletions
    }
}

fn main() {
    let s = "aabb".to_string();
    let k = 2;
    let result = Solution::min_deletions(s, k);
    println!("Minimum deletions: {}", result); // Output: 2
}
