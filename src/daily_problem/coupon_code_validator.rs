struct Solution;

impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        fn is_valid_str(str: &str) -> bool {
          if str == "" {
            return false;
          }
          str.chars().all(|c| {
            c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit()
          })
        }

        fn is_valid_business_line(str: &str) -> bool {
          str == "electronics" || str ==  "grocery" || str == "pharmacy" || str == "restaurant"
        }

        let mut coupons: Vec<String> = Vec::new();
        for i in 0..code.len() {
          if is_active[i] && is_valid_business_line(business_line[i].as_str()) && is_valid_str(code[i].as_str()) {
            println!("{i} | {}", code[i]);
            coupons.push(format!("{}{}", business_line[i].chars().next().unwrap(), code[i]));
          }
        }
        coupons.sort();
        coupons.iter_mut().for_each(|c| {c.remove(0);});
        coupons
    }
}
