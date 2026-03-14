struct Solution;

impl Solution {
    pub fn largest_palindrome(num: String) -> String {
        let mut map = vec![0; 10];
        for ch in num.chars() {
            map[ch as usize - '0' as usize] += 1;
        }

        let mut ans = String::new();
        let mut mid = -1;
        for i in (0..=9).rev() {
            if map[i] == 0 {
                continue;
            }

            if map[i] % 2 == 1 {
                mid = mid.max(i as i32);
            }

            if i == 0 && ans.is_empty() {
                break;
            }

            let mut t = map[i] / 2;
            while t > 0 {
                ans.push((i as u8 + '0' as u8) as char);
                t -= 1;
            }
        }

        let rev = ans.chars().rev().collect::<String>();
        if mid != -1 {
            ans.push((mid as u8 + '0' as u8) as char);
        }
        ans.push_str(&rev);
        return if !ans.is_empty() {
            ans
        } else {
            "0".to_string()
        };
    }
}
