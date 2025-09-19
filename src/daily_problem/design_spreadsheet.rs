use std::collections::HashMap;
struct Spreadsheet {
    sheet: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Spreadsheet {
            sheet: HashMap::new(),
        }
    }
    fn set_cell(&mut self, cell: String, value: i32) {
        self.sheet.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        if let Some(v) = self.sheet.get(&cell) {
            self.sheet.remove(&cell);
        }
    }

    fn get_value(&self, formula: String) -> i32 {
        let cell = formula[1..].split('+');
        let mut ans = 0;
        for c in cell {
            if let Some(v) = self.sheet.get(c) {
                ans += v;
            }
            if let Ok(v) = c.parse::<i32>() {
                ans += v;
            }
        }
        ans
    }
}
