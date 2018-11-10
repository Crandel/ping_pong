use ball::{Ball, Direction};
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

impl Manager {
    pub fn new() -> Manager {
        let term = Term::stdout();
        let (h, w) = term.size();

        let ball = Ball::new(w / 2, h / 2);
        let player1 = Paddle::new(1, h / 2);
        let player2 = Paddle::new(w - 2, h / 2);
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
        let horizontal_border = str::repeat("#", self.widht as usize);
        self.term.write_line(&horizontal_border).unwrap();
        for x in 0..self.widht {
            for y in 0..self.height {}
        }
        self.term.write_line(&horizontal_border).unwrap();
    }

    pub fn start(&mut self) -> io::Result<()> {
        self.term.clear_last_lines(self.height as usize)?;
        while !self.quit {
            self.draw();
            let user_input = self.term.read_char().unwrap();
            if user_input == 'q' {
                self.quit = true;
            }
        }
        self.term.clear_last_lines(self.height as usize)?;
        Ok(())
    }
}
