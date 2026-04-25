struct Solution;

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut ans = 0;

        let max_square = {
            let mut x = 0;
            let mut y = 0;

            for point in &points {
                x = point[0].abs().max(x);
                y = point[1].abs().max(y);
            }

            x.max(y) * 2
        };

        let mut left = 0;
        let mut right = max_square;

        while left <= right {
            let mid = left + (right - left) / 2;

            let mut freq = vec![0; 26];

            let mut flag = true;
            for (i, point) in points.iter().enumerate() {
                if point[0].abs() <= mid && point[1].abs() <= mid {
                    if freq[s.as_bytes()[i] as usize - 'a' as usize] > 0 {
                        flag = false;
                        break;
                    }
                    freq[s.as_bytes()[i] as usize - 'a' as usize] += 1;
                }
            }

            if flag {
                ans = ans.max(freq.iter().sum());
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        println!("max_square: {max_square}");

        ans
    }
}
