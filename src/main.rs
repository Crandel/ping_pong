extern crate ping_pong;
use ping_pong::ball::{Ball, Direction};

fn main() {
    let mut ball = Ball::new(0, 0);
    println!("Start moving in {:?}\n", Direction::DOWN_RIGHT);
    ball.change_direction(Direction::DOWN_RIGHT);
    ball.move_ball();
    println!("{}", ball);
    println!("\nNext move\n",);
    ball.move_ball();
    println!("{}", ball);
    println!("");
    println!("\nNext move in {:?}\n", Direction::RIGHT);
    ball.change_direction(Direction::RIGHT);
    ball.move_ball();
    println!("{}", ball);
    println!("");
    println!("\nNext move in {:?}\n", Direction::DOWN_LEFT);
    ball.change_direction(Direction::DOWN_LEFT);
    ball.move_ball();
    println!("{}", ball);
    println!("");
    println!("\nNext move\n");
    ball.move_ball();
    println!("{}", ball);
    println!("\nEnd of move");
}
