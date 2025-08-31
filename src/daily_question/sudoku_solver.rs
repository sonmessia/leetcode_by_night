struct Solution {}

impl Solution {
    fn is_valid(board: &mut Vec<Vec<char>>, i: usize, j: usize, ch: char) -> bool {
        for k in 0..9 {
            if board[i][k] == ch {
                return false;
            }
            if board[k][j] == ch {
                return false;
            }

            let sub_row = 3 * (i / 3) + k / 3;
            let sub_col = 3 * (j / 3) + k % 3;
            if board[sub_row][sub_col] == ch {
                return false;
            }
        }
        true
    }
    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    for ch in '1'..='9' {
                        if Self::is_valid(board, i, j, ch) {
                            board[i][j] = ch;
                            if Self::solve(board) {
                                return true;
                            }
                            board[i][j] = '.';
                        }
                    }
                    return false;
                }
            }
        }

        true
    }
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }
}

fn main() {
    let mut board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
        Solution::solve_sudoku(&mut board);
        println!("{:?}", board);
}
