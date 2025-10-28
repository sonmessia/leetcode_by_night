struct Solution;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        fn calculate_score(rolls: &Vec<i32>) -> i32 {
            let mut score = 0;
            for i in 0..rolls.len() {
                score += rolls[i];
                if i == 1 && rolls[i - 1] == 10 {
                    score += rolls[i];
                }
                if i >= 2 && (rolls[i - 2] == 10 || rolls[i - 1] == 10) {
                    score += rolls[i];
                }
            }
            score
        }

        let score1 = calculate_score(&player1);
        let score2 = calculate_score(&player2);

        if score1 > score2 {
            1
        } else if score2 > score1 {
            2
        } else {
            0
        }
    }
}
