use std::cmp::Ordering;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let mut rank_count: HashMap<char, HashMap<i32, i32>> = HashMap::new();

        for vote in &votes {
            for (i, ch) in vote.chars().enumerate() {
                let entry = rank_count.entry(ch).or_insert_with(HashMap::new);
                *entry.entry(i as i32).or_insert(0) += 1;
            }
        }

        let mut ans: String = String::new();
        let mut rank_vec: Vec<(char, HashMap<i32, i32>)> = rank_count.into_iter().collect();
        println!("{:?}", rank_vec);
        rank_vec.sort_by(|a, b| {
            for i in 0..votes[0].len() as i32 {
                let count_a = a.1.get(&i).unwrap_or(&0);
                let count_b = b.1.get(&i).unwrap_or(&0);
                match count_b.cmp(count_a) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq,
                }
            }
            a.0.cmp(&b.0)
        });
        for (ch, _) in rank_vec {
            ans.push(ch);
        }
        ans
    }
}

fn main() {
    let votes = vec![
        "WXYZ".to_string(),
        "XYZW".to_string(),
        "YZWX".to_string(),
        "ZWXY".to_string(),
    ];
    let result = Solution::rank_teams(votes);
    println!("{}", result); // Output: "XWYZ"
}
