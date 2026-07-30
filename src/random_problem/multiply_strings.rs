struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let mut result = vec![0; num1.len() + num2.len()];

        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let mul = (num1[i] - b'0') as usize * (num2[j] - b'0') as usize;
                let sum = mul + result[i + j + 1];
                result[i + j + 1] = sum % 10;
                result[i + j] += sum / 10;
            }
        }

        let mut result_str = String::new();
        for &digit in &result {
            if !(result_str.is_empty() && digit == 0) {
                result_str.push((digit as u8 + b'0') as char);
            }
        }

        if result_str.is_empty() {
            "0".to_string()
        } else {
            result_str
        }
    }
}
