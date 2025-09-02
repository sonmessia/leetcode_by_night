struct Solution;

impl Solution {
    pub fn number_of_pairs(mut p: Vec<Vec<i32>>) -> i32 {
        p.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });

        let n = p.len();
        let mut ans = 0;

        for i in 0..n.saturating_sub(1) {
            let mut y = i32::MAX;

            for j in (i + 1)..n {
                if p[j][1] >= p[i][1] && y > p[j][1] {
                    ans += 1;
                    y = p[j][1];
                }
            }
        }
        ans
    }
}

fn main() {
    let p = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![5, 6]];
    let result = Solution::number_of_pairs(p);
    println!("Number of valid pairs: {}", result); // Output: 0
}
