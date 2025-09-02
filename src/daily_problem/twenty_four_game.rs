struct Solution;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        // dfs, backtracking
        let nums = cards.into_iter().map(|x| x as f64).collect();
        fn dfs(nums: Vec<f64>) -> bool {
            const EPS: f64 = 1e-6;
            let n = nums.len();
            if n == 1 {
                return (nums[0] - 24.0).abs() < EPS;
            }

            for i in 0..n {
                for j in (i + 1)..n {
                    let mut next = Vec::with_capacity(n - 1);
                    for k in 0..n {
                        if k != i && k != j {
                            next.push(nums[k]);
                        }
                    }

                    let (a, b) = (nums[i], nums[j]);

                    let mut candidates = vec![a + b, a - b, b - a, a * b];
                    if b.abs() > EPS {
                        candidates.push(a / b);
                    }
                    if a.abs() > EPS {
                        candidates.push(b / a);
                    }

                    for val in candidates {
                        let mut new_nums = next.clone();
                        new_nums.push(val);
                        if dfs(new_nums) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        dfs(nums)
    }
}
fn main() {
    // Example usage:
    let result = Solution::judge_point24(vec![8, 1, 6, 6]);
    println!("Can make 24: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_point24() {
        assert!(Solution::judge_point24(vec![8, 1, 6, 6]));
        assert!(!Solution::judge_point24(vec![1, 2, 1, 2]));
    }
}
