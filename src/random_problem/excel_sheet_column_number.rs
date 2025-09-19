struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut column_title = column_title;
        let mut result = 0;
        let mut multiplier = 1;
        while !column_title.is_empty() {
            let last_char = column_title.pop().unwrap();
            let value = (last_char as u8 - b'A' + 1) as i32;
            result += value * multiplier;
            multiplier *= 26;
        }
        result
    }
}
