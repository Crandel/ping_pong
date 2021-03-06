use rand::Rng;

use crate::ball::{Ball, Direction};
use crate::paddle::Paddle;

pub struct Manager {
    widht: i32,
    height: i32,
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
    score1: u32,
    score2: u32,
    quit: bool,
}

const OFFSET_PADDLE: i32 = 2;
const OFFSET_WITH_PADDLE_LENGTH: i32 = 8;
const HORIZONTAL_B_CHAR: &str = "-";
const VERTICAL_B_CHAR: &str = "|";
const BALL_CHAR: &str = "@";
const PLAYER_CHAR: &str = "$";
const DIRECTIONS: [Direction; 6] = [
    Direction::Left,
    Direction::UpLeft,
    Direction::DownLeft,
    Direction::Right,
    Direction::UpRight,
    Direction::DownRight,
];

impl Manager {
    pub fn new(height: i32, widht: i32) -> Manager {
        let ball_symb: String = String::from(BALL_CHAR);
        let player1_symb: String = String::from(PLAYER_CHAR);
        let player2_symb: String = String::from(PLAYER_CHAR);

        // ball in the middle of the field
        let ball = Ball::new(widht / 2, height / 2, ball_symb);
        // Left player
        let player1 = Paddle::new(1, height / 2 - OFFSET_PADDLE, player1_symb);
        // Right player
        let player2 = Paddle::new(
            widht - OFFSET_PADDLE,
            height / 2 - OFFSET_PADDLE,
            player2_symb,
        );

        Manager {
            widht,
            height,
            ball,
            player1,
            player2,
            score1: 0,
            score2: 0,
            quit: false,
        }
    }

    pub fn get_quit(&self) -> bool {
        self.quit
    }
    fn score_up(&mut self, player: bool) {
        if player {
            self.score1 += 1;
        } else {
            self.score2 += 1;
        }
        self.ball.reset();
        self.player1.reset();
        self.player2.reset();
    }

    fn form_paddle(&self, player_y: i32, i: i32) -> bool {
        player_y == i
            || player_y + 1 == i
            || player_y + 2 == i
            || player_y + 3 == i
            || player_y + 4 == i
    }

    pub fn draw(&self) -> Vec<Vec<String>> {
        let mut draw_box: Vec<Vec<String>> = Vec::new();
        let mut horizontal_border = Vec::new();
        for _i in 0..self.widht {
            horizontal_border.push(String::from(HORIZONTAL_B_CHAR));
        }
        draw_box.push(horizontal_border.clone());
        // need 3 lines for borders and help string
        for i in 0..self.height - 3 {
            let mut line = Vec::new();
            for j in 0..self.widht {
                let ball_x = self.ball.get_x();
                let ball_y = self.ball.get_y();
                let player1_x = self.player1.get_x();
                let player1_y = self.player1.get_y();
                let player2_x = self.player2.get_x();
                let player2_y = self.player2.get_y();
                // left border
                // right border
                if j == 0 && j == self.widht - 1 {
                    line.push(String::from(VERTICAL_B_CHAR));

                // playground
                // Ball
                } else if ball_x == j && ball_y == i {
                    line.push(self.ball.get_symbol());
                // create 5 char paddle for player 1
                } else if player1_x == j && self.form_paddle(player1_y, i) {
                    line.push(self.player1.get_symbol());
                // create 5 char paddle for player 2
                } else if player2_x == j && self.form_paddle(player2_y, i) {
                    line.push(self.player2.get_symbol());
                } else {
                    line.push(String::from(" "));
                }
            }
            draw_box.push(line);
        }
        draw_box.push(horizontal_border);
        let mut bottom_str = String::with_capacity(self.widht as usize);
        bottom_str.push_str(&format!(
            "score 1: {} | score 2: {} | Press space for start | Q for exit | W (up) and S (down) for player 1 | I (up) and K (down) for player 2",
            self.score1, self.score2));
        let mut bottom_vec: Vec<String> = Vec::new();
        for chr in bottom_str.chars() {
            bottom_vec.push(chr.to_string());
        }
        draw_box.push(bottom_vec);
        draw_box
    }

    pub fn controls(&mut self, ch_res: Option<i32>) {
        if let Some(ch) = ch_res {
            match ch {
                113 => self.quit = true,
                119 => {
                    if self.player1.get_y() > 0 {
                        self.player1.move_up();
                    }
                }
                115 => {
                    if self.player1.get_y() < self.height - OFFSET_WITH_PADDLE_LENGTH {
                        self.player1.move_down();
                    }
                }
                105 => {
                    if self.player2.get_y() > 0 {
                        self.player2.move_up();
                    }
                }
                107 => {
                    if self.player2.get_y() < self.height - OFFSET_WITH_PADDLE_LENGTH {
                        self.player2.move_down();
                    }
                }
                32 => {
                    if *self.ball.get_direction() == Direction::Stop {
                        let mut rng = rand::thread_rng();
                        let dir: &Direction = rng.choose(&DIRECTIONS).unwrap();
                        self.ball.change_direction(dir.clone());
                    }
                }
                _ => {}
            };
        }
    }

    pub fn ball_movements(&mut self) {
        let ball_x = self.ball.get_x();
        let ball_y = self.ball.get_y();
        let player1_x = self.player1.get_x();
        let player1_y = self.player1.get_y();
        let player2_x = self.player2.get_x();
        let player2_y = self.player2.get_y();

        for i in 0..5 {
            // left paddle
            if ball_x == player1_x + 1 && ball_y == player1_y + i {
                // println!("left hit");
                let mut rng = rand::thread_rng();
                let dir: &Direction = rng.choose(&DIRECTIONS[4..6]).unwrap();
                self.ball.change_direction(dir.clone())
            }
            // right paddle
            if ball_x == player2_x - 1 && ball_y == player2_y + i {
                let mut rng = rand::thread_rng();
                let dir: &Direction = rng.choose(&DIRECTIONS[0..3]).unwrap();
                self.ball.change_direction(dir.clone())
            }
        }

        // top wall
        if ball_y == 0 {
            let next_dir = if *self.ball.get_direction() == Direction::UpRight {
                Direction::DownRight
            } else {
                Direction::DownLeft
            };
            self.ball.change_direction(next_dir);
        }

        // bottom wall (skip the help string and border)
        if ball_y == self.height - 4 {
            let next_dir = if *self.ball.get_direction() == Direction::DownRight {
                Direction::UpRight
            } else {
                Direction::UpLeft
            };
            self.ball.change_direction(next_dir);
        }

        // right wall
        if ball_x == self.widht - 1 {
            self.score_up(true);
        }

        // left wall
        if ball_x == 0 {
            self.score_up(false);
        }

        self.ball.move_ball();
    }

    pub fn is_ball(&self, chr: &str) -> bool {
        chr == BALL_CHAR
    }

    pub fn is_paddle(&self, chr: &str) -> bool {
        chr == PLAYER_CHAR
    }
}
