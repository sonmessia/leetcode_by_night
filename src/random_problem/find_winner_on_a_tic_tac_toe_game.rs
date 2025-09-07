struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec![vec![' '; 3]; 3];
        let mut isA = true;
        for mv in moves.iter() {
            board[mv[0] as usize][mv[1] as usize] = if isA { 'X' } else { 'O' };
            isA = !isA;
        }
        for i in 0..3 {
            if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
                return if board[i][0] == 'X' {
                    "A".to_string()
                } else {
                    "B".to_string()
                };
            }
            if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
                return if board[0][i] == 'X' {
                    "A".to_string()
                } else {
                    "B".to_string()
                };
            }
        }
        if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
            return if board[0][0] == 'X' {
                "A".to_string()
            } else {
                "B".to_string()
            };
        }

        if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
            return if board[0][2] == 'X' {
                "A".to_string()
            } else {
                "B".to_string()
            };
        }

        if moves.len() as i32 == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}

fn main() {}
