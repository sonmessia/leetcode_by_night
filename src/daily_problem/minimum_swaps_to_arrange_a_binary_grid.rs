struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut count = vec![0; n];
        for i in 0..n {
            for j in (0..n).rev() {
                if grid[i][j] == 1 {
                    break;
                }
                count[i] += 1;
            }
        }

        println!("{:?}", count);

        let (mut ans, mut value) = (0, n - 1);

        for i in 0..n {
            let mut j = i;
            while j < n && count[j] < value {
                j += 1;
            }

            if j == n {
                return -1;
            }

            for k in (i..=j).rev() {
                if k <= 0 {
                    continue;
                }
                println!("{}", k);
                count.swap(k, k - 1);
            }

            ans += j - i;
            value -= 1;

            if value == 0 {
                break;
            }
        }

        println!("{:?}", count);

        ans as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_swaps(vec![
            vec![1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0],
            vec![1, 1, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0]
        ])
    );
}
