struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];

        for row in 0..9 {
            for col in 0..9 {
                let ch = board[row][col];

                if ch == '.' {
                    continue;
                }

                let num = (ch as u8 - b'1') as usize; // '1' -> 0, '9' -> 8
                let box_idx = (row / 3) * 3 + (col / 3);

                if rows[row][num] || cols[col][num] || boxes[box_idx][num] {
                    return false;
                }

                rows[row][num] = true;
                cols[col][num] = true;
                boxes[box_idx][num] = true;
            }
        }

        true
    }
}

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let is_valid_sudoku = Solution::is_valid_sudoku(board);
    println!("{}", is_valid_sudoku);
}
