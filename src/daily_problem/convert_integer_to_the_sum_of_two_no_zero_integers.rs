struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            let a = i.to_string();
            let b = (n - i).to_string();
            if !a.contains('0') && !b.contains('0') {
                return vec![i, n - i];
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_no_zero_integers() {
        assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
        assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
        assert_eq!(Solution::get_no_zero_integers(10000), vec![1, 9999]);
        assert_eq!(Solution::get_no_zero_integers(69), vec![1, 68]);
        assert_eq!(Solution::get_no_zero_integers(1010), vec![11, 999]);
    }
}
