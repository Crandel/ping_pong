pub struct Paddle {
    x: u8,
    y: u8,
    orig_x: u8,
    orig_y: u8,
}

impl Paddle {
    pub fn new(x: u8, y: u8) -> Paddle {
        Paddle {
            x: x,
            y: y,
            orig_x: x,
            orig_y: y,
        }
    }

    pub fn reset(&mut self) {
        self.x = self.orig_x;
        self.y = self.orig_y;
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }
}

impl std::fmt::Display for Paddle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
