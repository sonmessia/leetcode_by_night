struct Solution;

impl Solution {
    pub fn dist_money(mut money: i32, mut children: i32) -> i32 {
        if money < children {
            return -1;
        }

        money -= children;

        let mut ans = 0;

        while money >= 7 && children > 0 {
            money -= 7;
            children -= 1;
            ans += 1;
        }

        if ans > 0 {
            if children == 0 && money > 0 {
                ans -= 1;
            }
            if children == 1 && money == 3 {
                ans -= 1;
            }
        }
        ans
    }
}
