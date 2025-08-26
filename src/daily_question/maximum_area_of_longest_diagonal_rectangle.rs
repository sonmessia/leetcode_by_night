struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut max_dia_sq = 0;

        for d in dimensions {
            let (l, w) = (d[0], d[1]);
            let dia_sq = l * l + w * w;
            let area = l * w;
            if dia_sq > max_dia_sq {
                max_dia_sq = dia_sq;
                ans = area;
            } else if dia_sq == max_dia_sq {
                ans = ans.max(area);
            }
        }
        ans
    }
}

fn main() {
    let dimensions = vec![vec![9, 3], vec![3, 4], vec![5, 6], vec![8, 6]];
    let result = Solution::area_of_max_diagonal(dimensions);
    println!("The area of the rectangle with the maximum diagonal is: {}", result);
}