use std::io;
use std::io::Read;
use std::thread;
use std::time::Duration;

use console::{style, Term};
use rand::Rng;
use termion::async_stdin;

use ball::{Ball, Direction};
use paddle::Paddle;

pub struct Manager {
    term: Term,
    widht: u16,
    height: u16,
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
    score1: u32,
    score2: u32,
    quit: bool,
}

const HORIZONTAL_B_CHAR: &str = "-";
const VERTICAL_B_CHAR: &str = "|";
const BALL_CHAR: &str = "0";
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
    pub fn new() -> Manager {
        let term = Term::stdout();
        let (h, w) = term.size();
        let height = h - 4;
        let widht = w - 2;
        let ball_symb: String = format!("{}", style(BALL_CHAR).red().on_black().bold());
        let player1_symb: String = format!("{}", style(PLAYER_CHAR).green().on_black().bold());
        let player2_symb: String = format!("{}", style(PLAYER_CHAR).blue().on_black().bold());

        let ball = Ball::new(widht / 2, height / 2, ball_symb);
        let player1 = Paddle::new(0, height / 2 - 2, player1_symb);
        let player2 = Paddle::new(widht - 1, height / 2 - 2, player2_symb);

        Manager {
            term: term,
            widht: widht,
            height: height,
            ball: ball,
            player1: player1,
            player2: player2,
            score1: 0,
            score2: 0,
            quit: false,
        }
    }

    pub fn score_up(&mut self, player: bool) {
        if player {
            self.score1 += 1;
        } else {
            self.score2 += 1;
        }
        self.ball.reset();
        self.player1.reset();
        self.player2.reset();
    }

    fn form_paddle(&self, player_y: u16, i: u16) -> bool {
        player_y == i
            || player_y + 1 == i
            || player_y + 2 == i
            || player_y + 3 == i
            || player_y + 4 == i
    }

    fn draw(&self) {
        let horizontal_border = str::repeat(HORIZONTAL_B_CHAR, self.widht as usize + 1);
        self.term.write_line(&horizontal_border).unwrap();
        for i in 0..self.height {
            let mut line = String::from("");
            for j in 0..self.widht {
                let ball_x = self.ball.get_x();
                let ball_y = self.ball.get_y();
                let player1_x = self.player1.get_x();
                let player1_y = self.player1.get_y();
                let player2_x = self.player2.get_x();
                let player2_y = self.player2.get_y();
                // left border
                if j == 0 {
                    line.push_str(VERTICAL_B_CHAR);
                }

                // playground
                // Ball
                if ball_x == j && ball_y == i {
                    line.push_str(self.ball.get_symbol());
                // create 5 char paddle for player 1
                } else if player1_x == j && self.form_paddle(player1_y, i) {
                    line.push_str(self.player1.get_symbol());
                // create 5 char paddle for player 2
                } else if player2_x == j && self.form_paddle(player2_y, i) {
                    line.push_str(self.player2.get_symbol());
                } else {
                    line.push_str(" ");
                }

                // right border
                if j == self.widht - 1 {
                    line.push_str(VERTICAL_B_CHAR);
                }
            }
            self.term.write_line(&line).unwrap();
        }
        self.term.write_line(&horizontal_border).unwrap();
        self.term
            .write_line(&format!(
                "score 1: {} | score2: {}      W ⇑ ans S ⇓ for player 1 | I ⇑ and K ⇓ for player2 | Q for exit",
                self.score1, self.score2,
            )).unwrap();
    }

    fn controls(&mut self, ui: u8) {
        match ui {
            b'q' => self.quit = true,
            b'w' => {
                if self.player1.get_y() > 0 {
                    self.player1.move_up();
                }
            }
            b's' => {
                if self.player1.get_y() < self.height - 1 {
                    self.player1.move_down();
                }
            }
            b'i' => {
                if self.player2.get_y() > 0 {
                    self.player2.move_up();
                }
            }
            b'k' => {
                if self.player2.get_y() < self.height - 1 {
                    self.player2.move_down();
                }
            }
            b' ' => {
                if *self.ball.get_direction() == Direction::Stop {
                    let mut rng = rand::thread_rng();
                    let dir: &Direction = rng.choose(&DIRECTIONS).unwrap();
                    self.ball.change_direction(dir.clone());
                }
            }
            _ => {}
        }
    }

    fn ball_movements(&mut self) {
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

        // bottom wall
        if ball_y == self.height - 1 {
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

    pub fn start(&mut self) -> io::Result<()> {
        let mut stdin = async_stdin();

        let terminal_height = self.height as usize;
        self.term.clear_last_lines(terminal_height)?;
        while !self.quit {
            self.draw();
            let mut key_bytes = [0; 1];
            stdin.read(&mut key_bytes).unwrap();
            self.controls(key_bytes[0]);
            self.ball_movements();
            thread::sleep(Duration::from_millis(100));
            self.term.clear_last_lines(terminal_height)?;
        }
        Ok(())
    }
}
