#[derive(PartialEq, Clone)]
pub struct Paddle {
    x: u16,
    y: u16,
    orig_x: u16,
    orig_y: u16,
    symbol: String,
}

impl Paddle {
    pub fn new(x: u16, y: u16, s: String) -> Paddle {
        Paddle {
            x: x,
            y: y,
            orig_x: x,
            orig_y: y,
            symbol: s,
        }
    }

    pub fn reset(&mut self) {
        self.x = self.orig_x;
        self.y = self.orig_y;
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
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
