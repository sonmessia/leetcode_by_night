struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut result = String::new();
        let mut carry = 0;

        let num1_chars = num1.chars().rev().collect::<Vec<char>>();
        let num2_chars = num2.chars().rev().collect::<Vec<char>>();

        let (mut i, mut j) = (0, 0);

        while i < num1_chars.len() || j < num2_chars.len() || carry > 0 {
            let digit1 = if i < num1_chars.len() {
                num1_chars[i].to_digit(10).unwrap()
            } else {
                0
            };

            let digit2 = if j < num2_chars.len() {
                num2_chars[j].to_digit(10).unwrap_or(0)
            } else {
                0
            };

            let sum = digit1 + digit2 + carry;
            carry = sum / 10;
            result.push(std::char::from_digit(sum % 10, 10).unwrap());

            i += 1;
            j += 1;
        }

        result.chars().rev().collect()
    }
}
