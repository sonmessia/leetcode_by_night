struct ProductOfNumbers {
    numbers: Vec<i32>,
    last_idx_of_zero: i32,

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        return Self {numbers: vec![1],last_idx_of_zero: -1};
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.numbers.clear();
            self.numbers.push(1);
            self.last_idx_of_zero = self.numbers.len() as i32 - 1;
        } else {
            let last_product = *self.numbers.last().unwrap();
            self.numbers.push(last_product * num);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        if k >= self.numbers.len() as i32 {
            return 0;
        } 
        return self.numbers.last().unwrap() / self.numbers[self.numbers.len() - 1 - k as usize];
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
