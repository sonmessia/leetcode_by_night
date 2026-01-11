struct Solution;

impl Solution {
    fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        let mut extended_heights = heights.clone();
        extended_heights.push(0); // Sentinel

        for (i, &h) in extended_heights.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if h < extended_heights[top] {
                    stack.pop();
                    let width = if let Some(&new_top) = stack.last() {
                        i - new_top - 1
                    } else {
                        i
                    };
                    let area = extended_heights[top] * width as i32;
                    max_area = max_area.max(area);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        max_area
    }
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut heights = vec![0; cols];
        let mut max_area = 0;

        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == '1' {
                    heights[c] += 1;
                } else {
                    heights[c] = 0;
                }
            }
            max_area = max_area.max(Self::largest_rectangle_area(&heights));
        }

        max_area
    }
}
