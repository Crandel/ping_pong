extern crate ping_pong;
use ping_pong::manager::Manager;

fn main() {
    let mut manager = Manager::new();
    manager.start().unwrap();
    // let mut ball = Ball::new(0, 0);
    // println!("Start moving ball in {:?}\n", Direction::DownRight);
    // ball.change_direction(Direction::DownRight);
    // ball.move_ball();
    // println!("{}", ball);
    // println!("\nNext move\n",);
    // ball.move_ball();
    // println!("{}", ball);
    // println!("");
    // println!("\nNext move in {:?}\n", Direction::Right);
    // ball.change_direction(Direction::Right);
    // ball.move_ball();
    // println!("{}", ball);
    // println!("");
    // println!("\nNext move in {:?}\n", Direction::DownLeft);
    // ball.change_direction(Direction::DownLeft);
    // ball.move_ball();
    // println!("{}", ball);
    // println!("");
    // println!("\nNext move\n");
    // ball.move_ball();
    // println!("{}", ball);
    // println!("\nEnd of ball move");

    // let mut paddle1 = Paddle::new(0, 5);
    // let mut paddle2 = Paddle::new(10, 5);

    // println!("Start moving paddle1 down \n");
    // paddle1.move_down();
    // println!("{}", paddle1);
    // println!("\nNext move down\n");
    // paddle1.move_down();
    // println!("{}", paddle1);
    // println!("\nNext move up\n");
    // paddle1.move_up();
    // println!("{}", paddle1);
    // println!("\nEnd of paddle1 move");

    // println!("Start moving paddle2 down \n");
    // paddle2.move_down();
    // println!("{}", paddle2);
    // println!("\nNext move up\n");
    // paddle2.move_up();
    // println!("{}", paddle2);
    // println!("\nNext move down\n");
    // paddle2.move_down();
    // println!("{}", paddle2);
    // println!("\nEnd of paddle2 move");
}
