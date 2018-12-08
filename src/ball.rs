#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Stop,
    Left,
    UpLeft,
    DownLeft,
    Right,
    UpRight,
    DownRight,
}

pub struct Ball {
    x: i32,
    y: i32,
    orig_x: i32,
    orig_y: i32,
    d: Direction,
    symbol: String,
}

impl Ball {
    pub fn new(x: i32, y: i32, s: String) -> Ball {
        Ball {
            x,
            y,
            orig_x: x,
            orig_y: y,
            d: Direction::Stop,
            symbol: s,
        }
    }

    pub fn reset(&mut self) {
        self.x = self.orig_x;
        self.y = self.orig_y;
        self.d = Direction::Stop;
    }

    pub fn change_direction(&mut self, d: Direction) {
        self.d = d;
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

    pub fn get_direction(&self) -> &Direction {
        &self.d
    }

    pub fn move_ball(&mut self) {
        match self.d {
            Direction::Left => self.x -= 1,
            Direction::UpLeft => {
                self.x -= 1;
                self.y -= 1;
            }
            Direction::DownLeft => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::Right => self.x += 1,
            Direction::UpRight => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::DownRight => {
                self.x += 1;
                self.y += 1;
            }
            _ => {}
        }
    }
}

impl std::fmt::Display for Ball {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, d: {:?}", self.x, self.y, self.d)
    }
}
