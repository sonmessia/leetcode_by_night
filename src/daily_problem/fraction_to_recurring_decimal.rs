struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let mut result = String::new();
        // Determine the sign
        if (numerator < 0) ^ (denominator < 0) {
            result.push('-');
        }
        let num = (numerator as i64).abs();
        let den = (denominator as i64).abs();
        // Integer part
        result.push_str(&(num / den).to_string());
        let mut remainder = num % den;
        if remainder == 0 {
            return result;
        }
        result.push('.');
        let mut map = std::collections::HashMap::new();
        let mut decimal_part = String::new();
        let mut index = 0;
        while remainder != 0 {
            if let Some(&pos) = map.get(&remainder) {
                decimal_part.insert_str(pos, "(");
                decimal_part.push(')');
                break;
            }
            map.insert(remainder, index);
            remainder *= 10;
            decimal_part.push_str(&(remainder / den).to_string());
            remainder %= den;
            index += 1;
        }
        result.push_str(&decimal_part);
        result
    }
}
