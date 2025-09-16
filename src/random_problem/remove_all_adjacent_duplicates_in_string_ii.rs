struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();
        for c in s.chars() {
            println!("Before process {:?}", stack);
            if let Some(last) = stack.last_mut() {
                if last.0 == c {
                    last.1 += 1;
                    if last.1 == k {
                        stack.pop();
                    }
                } else {
                    stack.push((c, 1));
                }
            } else {
                stack.push((c, 1));
            }
            println!("After {:?}", stack);
        }
        let mut result = String::new();
        for (c, count) in stack {
            for _ in 0..count {
                result.push(c);
            }
        }
        result
    }
}

fn main() {
    let s = "deeedbbcccbdaa".to_string();
    let k = 3;
    let result = Solution::remove_duplicates(s, k);
    println!("{}", result); // Output: "aa"
}
