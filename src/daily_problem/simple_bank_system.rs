struct Bank {
    balances: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balances: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1 > self.balances.len() as i32
            || account2 > self.balances.len() as i32
            || self.balances[account1 as usize - 1] < money
        {
            return false;
        }

        self.balances[account1 as usize - 1] -= money;
        self.balances[account2 as usize - 1] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account > self.balances.len() as i32 {
            return false;
        }
        self.balances[account as usize - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account > self.balances.len() as i32 || self.balances[account as usize - 1] < money {
            return false;
        }
        self.balances[account as usize - 1] -= money;
        true
    }
}
