use ball::Ball;
use paddle::Paddle;

use std::io;

use console::{style, Term};

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

impl Manager {
    pub fn new() -> Manager {
        let term = Term::stdout();
        let (h, w) = term.size();
        let ball_symb: String = format!("{}", style(BALL_CHAR).red().on_black().bold());
        let player1_symb: String = format!("{}", style(PLAYER_CHAR).green().on_black().bold());
        let player2_symb: String = format!("{}", style(PLAYER_CHAR).blue().on_black().bold());

        let ball = Ball::new(1, h / 2, ball_symb);
        let player1 = Paddle::new(0, h / 2, player1_symb);
        let player2 = Paddle::new(w - 3, h / 2, player2_symb);

        Manager {
            term: term,
            widht: w,
            height: h,
            ball: ball,
            player1: player1,
            player2: player2,
            score1: 0,
            score2: 0,
            quit: false,
        }
    }

    pub fn score_up(&mut self, player: Paddle) {
        if player == self.player1 {
            self.score1 += 1;
        }
        if player == self.player2 {
            self.score2 += 1;
        }
        self.ball.reset();
        self.player1.reset();
        self.player2.reset();
    }

    pub fn draw(&self) {
        let horizontal_border = str::repeat(HORIZONTAL_B_CHAR, self.widht as usize);
        self.term.write_line(&horizontal_border).unwrap();
        for i in 0..self.height - 4 {
            let mut line = String::from("");
            for j in 0..self.widht - 1 {
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
                if ball_x == j && ball_y == i {
                    line.push_str(self.ball.get_symbol());
                } else if player1_x == j && player1_y == i {
                    line.push_str(self.player1.get_symbol());
                } else if player2_x == j && player2_y == i {
                    line.push_str(self.player2.get_symbol());
                } else {
                    line.push_str(" ");
                }

                // right border
                if j == self.widht - 2 {
                    line.push_str("|");
                }
            }
            self.term.write_line(&line).unwrap();
        }
        self.term.write_line(&horizontal_border).unwrap();
    }

    pub fn start(&mut self) -> io::Result<()> {
        let terminal_height = self.height as usize;
        self.term.clear_last_lines(terminal_height)?;
        while !self.quit {
            self.draw();
            let user_input = self.term.read_char().unwrap();
            if user_input == 'q' {
                self.quit = true;
            }
            self.term.clear_last_lines(terminal_height)?;
        }
        Ok(())
    }
}
