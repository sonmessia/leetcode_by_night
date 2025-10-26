struct ATM {
    banknotes: [i32; 5],
    nominals: [i32; 5],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    fn new() -> Self {
        Self {
            banknotes: [0, 0, 0, 0, 0],
            nominals: [20, 50, 100, 200, 500],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, banknote_count) in banknotes_count.iter().enumerate() {
            self.banknotes[i] += banknote_count;
        }
    }

    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut ans = vec![0; 5];
        let mut ptr = self.banknotes.len() - 1;

        while let (0..=4) = ptr {
            if amount == 0 {
                return ans;
            }

            if amount < self.nominals[ptr] || self.banknotes[ptr] == 0 {
                ptr -= 1;
            } else {
                let decreament = self.banknotes[ptr].min(amount / self.nominals[ptr]);
                amount -= self.nominals[ptr] * decreament;
                self.banknotes[ptr] -= decreament;
                ans[ptr] += decreament;
            }
        }

        for (i, banknote_count) in ans.iter().enumerate() {
            self.banknotes[i] += banknote_count;
        }

        vec![-1]
    }
}
