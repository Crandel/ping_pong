#[derive(PartialEq, Clone)]
pub struct Paddle {
    x: i32,
    y: i32,
    orig_x: i32,
    orig_y: i32,
    symbol: String,
}

impl Paddle {
    pub fn new(x: i32, y: i32, s: String) -> Paddle {
        Paddle {
            x,
            y,
            orig_x: x,
            orig_y: y,
            symbol: s,
        }
    }

    pub fn reset(&mut self) {
        self.x = self.orig_x;
        self.y = self.orig_y;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
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
