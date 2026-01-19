struct NumArray {
    nums: Vec<i32>,
    bit: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut bit = vec![0; n + 1];
        for (i, &num) in nums.iter().enumerate() {
            Self::update_bit(&mut bit, i + 1, num);
        }
        Self { nums, bit }
    }

    fn update_bit(bit: &mut Vec<i32>, mut index: usize, delta: i32) {
        while index < bit.len() {
            bit[index] += delta;
            index += index & (!index + 1);
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        let delta = val - self.nums[index as usize];
        self.nums[index as usize] = val;
        Self::update_bit(&mut self.bit, (index + 1) as usize, delta);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sum((right + 1) as usize) - self.prefix_sum(left as usize)
    }

    fn prefix_sum(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        while index > 0 {
            sum += self.bit[index];
            index -= index & (!index + 1);
        }
        sum
    }
}
