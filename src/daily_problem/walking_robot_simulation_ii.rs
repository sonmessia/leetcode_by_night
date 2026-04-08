struct Robot {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    cur_direction: i32,
    direction_name: Vec<String>,
    perimeter: i32,
    has_moved: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            x: 0,
            y: 0,
            cur_direction: 0,
            direction_name: vec![
                "East".to_string(),
                "North".to_string(),
                "West".to_string(),
                "South".to_string(),
            ],
            perimeter: 2 * (width + height) - 4,
            has_moved: false,
        }
    }

    fn step(&mut self, num: i32) {
        let step = num % self.perimeter;
        // println!(
        //     "Current direction: {}, Current position: ({}, {}), current step: {}",
        //     self.direction_name[self.cur_direction as usize], self.x, self.y, step
        // );

        self.has_moved = true;

        match self.direction_name[self.cur_direction as usize].as_str() {
            "East" => {
                if self.x + step < self.width {
                    self.x += step;
                } else {
                    let remain = step - (self.width - 1 - self.x);
                    self.x = self.width - 1;
                    self.cur_direction = (self.cur_direction + 1) % 4;
                    self.step(remain);
                }
            }
            "North" => {
                if self.y + step < self.height {
                    self.y += step;
                } else {
                    let remain = step - (self.height - 1 - self.y);
                    self.y = self.height - 1;
                    self.cur_direction = (self.cur_direction + 1) % 4;
                    self.step(remain);
                }
            }
            "West" => {
                if self.x - step >= 0 {
                    self.x -= step;
                } else {
                    let remain = step - self.x;
                    self.x = 0;
                    self.cur_direction = (self.cur_direction + 1) % 4;
                    self.step(remain);
                }
            }
            "South" => {
                if self.y - step >= 0 {
                    self.y -= step;
                } else {
                    let remain = step - self.y;
                    self.y = 0;
                    self.cur_direction = (self.cur_direction + 1) % 4;
                    self.step(remain);
                }
            }
            _ => {}
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }

    fn get_dir(&self) -> String {
        if self.x == 0 && self.y == 0 && self.has_moved {
            return "South".to_string();
        }
        self.direction_name[self.cur_direction as usize].clone()
    }
}
