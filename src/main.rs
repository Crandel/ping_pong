extern crate ping_pong;
use ping_pong::ball::{Ball, Direction};

fn main() {
    let mut ball = Ball::new(0, 0);
    println!("Start moving in {:?}\n", Direction::DownRight);
    ball.change_direction(Direction::DownRight);
    ball.move_ball();
    println!("{}", ball);
    println!("\nNext move\n",);
    ball.move_ball();
    println!("{}", ball);
    println!("");
    println!("\nNext move in {:?}\n", Direction::Right);
    ball.change_direction(Direction::Right);
    ball.move_ball();
    println!("{}", ball);
    println!("");
    println!("\nNext move in {:?}\n", Direction::DownLeft);
    ball.change_direction(Direction::DownLeft);
    ball.move_ball();
    println!("{}", ball);
    println!("");
    println!("\nNext move\n");
    ball.move_ball();
    println!("{}", ball);
    println!("\nEnd of move");
}
