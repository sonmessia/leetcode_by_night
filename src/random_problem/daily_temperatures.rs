struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack = Vec::new();
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if temperatures[i] >= temperatures[top] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                result[i] = (top - i) as i32;
            }
            stack.push(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_daily_temperatures() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
        let temperatures = vec![30, 40, 50, 60];
        let expected = vec![1, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
        let temperatures = vec![30, 60, 90];
        let expected = vec![1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }
}
