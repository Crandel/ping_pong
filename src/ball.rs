#[derive(Debug)]
pub enum Direction {
    Stop,
    Left,
    UpLeft,
    DownLeft,
    Right,
    UpRight,
    DownRight,
}

#[derive(Debug)]
pub struct Ball {
    x: u8,
    y: u8,
    orig_x: u8,
    orig_y: u8,
    d: Direction,
}

impl Ball {
    pub fn new(x: u8, y: u8) -> Ball {
        Ball {
            x: x,
            y: y,
            orig_x: x,
            orig_y: y,
            d: Direction::Stop,
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

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
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
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
