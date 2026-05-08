struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];

        struct Item {
            value: i32,
            left: usize,
            right: usize,
        }

        let mut stack: Vec<Item> = Vec::new();
        for i in 0..n {
            let mut curr_item = Item {
                value: nums[i],
                left: i,
                right: i,
            };

            while let Some(top) = stack.last() {
                if top.value > nums[i] {
                    let top = stack.pop().unwrap();
                    curr_item.value = curr_item.value.max(top.value);
                    curr_item.left = top.left;
                } else {
                    break;
                }
            }
            stack.push(curr_item);
        }

        for i in 0..stack.len() {
            for j in stack[i].left..=stack[i].right {
                ans[j] = stack[i].value;
            }
        }

        ans
    }
}
