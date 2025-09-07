struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 1..=n / 2 {
            result.push(i);
            result.push(-i);
        }
        if n % 2 != 0 {
            result.push(0);
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_zero() {
        assert_eq!(Solution::sum_zero(5), vec![-2, -1, 0, 1, 2]);
        assert_eq!(Solution::sum_zero(3), vec![-1, 0, 1]);
        assert_eq!(Solution::sum_zero(1), vec![0]);
    }
}
